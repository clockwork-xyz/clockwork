use {
    crate::*,
    log::error,
    rdkafka::producer::{
        FutureProducer,
        FutureRecord,
        future_producer::OwnedDeliveryResult
    },
    prost::Message,
    std::{
        future::Future,
        time::Duration,
    },
    tokio::sync::{mpsc, oneshot},
};

pub struct Publisher {
    events_rx: mpsc::Receiver<Event>,
    finish_tx: Option<oneshot::Sender<()>>,

    producer: FutureProducer,
    produce_timeout: Duration,
    update_account_topic: String,
}

impl Publisher {
    /// Create new mpsc channel and publisher loop future.
    pub fn spawn(
        producer: FutureProducer,
        config: &Config,
    ) -> (PublisherHandle, impl Future<Output=()>) {
        // Channel to send events from plugin thread(s) to Publisher routine.
        let (events_tx, events_rx) = mpsc::channel(config.buffer_size);

        // One-shot channel to relay Publisher exit.
        let (finish_tx, finish_rx) = oneshot::channel::<()>();

        // Create publisher loop future.
        let publisher = Publisher {
            events_rx,
            finish_tx: Some(finish_tx),

            producer,
            produce_timeout: Duration::from_millis(config.produce_timeout_ms),
            update_account_topic: config.update_account_topic.clone(),
        };
        let fut = publisher.run();

        let handler = PublisherHandle { events_tx, finish_rx };
        (handler, fut)
    }

    async fn run(mut self) {
        while let Some(item) = self.events_rx.recv().await {
            let fut = match item {
                Event::UpdateAccount(ev) => self.produce_update_account(ev)
            };
            if let Err(e) = fut.await {
                error!("Failed to produce event: {:?}", e);
            }
        }
    }

    async fn produce_update_account(&self, ev: UpdateAccountEvent) -> OwnedDeliveryResult {
        let buf = ev.encode_to_vec();
        let record = FutureRecord::<(), _>::to(&self.update_account_topic)
            .payload(&buf);
        self.producer.send(record, self.produce_timeout).await
    }
}

impl Drop for Publisher {
    fn drop(&mut self) {
        // Send oneshot notice back to receiver to inform about successful closure.
        let _ = self.finish_tx
            .take()
            .expect("finish_tx disappeared")
            .send(());
    }
}

/// PublisherHandle allows communicating with the Publisher loop.
pub struct PublisherHandle {
    events_tx: mpsc::Sender<Event>,
    finish_rx: oneshot::Receiver<()>,
}

impl PublisherHandle {
    /// Send an event to the rdkafka Publisher loop.
    /// Blocks if the channel and producer buffers are full.
    pub fn blocking_send(&self, ev: Event) -> Result<(), mpsc::error::SendError<Event>> {
        self.events_tx.blocking_send(ev)
    }

    /// Signal graceful close.
    /// Blocks until the publisher loop has exited.
    pub fn blocking_close(self) {
        // Drop sender handle, signaling "close" to the publisher loop.
        drop(self.events_tx);
        // Wait until the publisher loop exits.
        let _ = self.finish_rx.blocking_recv();
    }
}
