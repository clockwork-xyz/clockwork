use {
    super::Manager,
    crate::errors::CronosError,
    anchor_lang::{prelude::*, AnchorDeserialize},
    chrono::{DateTime, NaiveDateTime, Utc},
    cronos_cron::Schedule,
    std::{convert::TryFrom, str::FromStr},
};

pub const SEED_QUEUE: &[u8] = b"queue";

// #[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
// pub enum Trigger {
//     Cron {
//         schedule: String,
//     },
//     Slot {
//         repeat: Option<u64>,
//         slot: Option<u64>,
//     },
// }

/**
 * Queue
 */

#[account]
#[derive(Debug)]
pub struct Queue {
    pub exec_at: Option<i64>,
    pub id: u128,
    pub manager: Pubkey,
    pub schedule: String,
    pub status: QueueStatus,
    pub task_count: u128,
    // pub trigger: Trigger,
}

impl Queue {
    pub fn pubkey(manager: Pubkey, id: u128) -> Pubkey {
        Pubkey::find_program_address(
            &[SEED_QUEUE, manager.as_ref(), id.to_be_bytes().as_ref()],
            &crate::ID,
        )
        .0
    }
}

impl TryFrom<Vec<u8>> for Queue {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Queue::try_deserialize(&mut data.as_slice())
    }
}

/**
 * QueueAccount
 */

pub trait QueueAccount {
    fn start(&mut self) -> Result<()>;

    fn new(
        &mut self,
        clock: &Sysvar<Clock>,
        manager: &mut Account<Manager>,
        schedule: String,
    ) -> Result<()>;

    fn next_exec_at(&self, ts: i64) -> Option<i64>;

    fn roll_forward(&mut self) -> Result<()>;
}

impl QueueAccount for Account<'_, Queue> {
    fn start(&mut self) -> Result<()> {
        // Validate the queue is pending
        require!(
            self.status == QueueStatus::Pending,
            CronosError::InvalidQueueStatus,
        );

        if self.task_count > 0 {
            // If there are actions, change the queue status to 'executing'
            self.status = QueueStatus::Processing { task_id: 0 };
        } else {
            // Otherwise, just roll forward the exec_at timestamp
            self.roll_forward()?;
        }

        Ok(())
    }

    fn new(
        &mut self,
        clock: &Sysvar<Clock>,
        manager: &mut Account<Manager>,
        schedule: String,
    ) -> Result<()> {
        // Initialize queue account
        self.id = manager.queue_count;
        self.manager = manager.key();
        self.schedule = schedule;
        self.status = QueueStatus::Pending;
        self.task_count = 0;

        // Set exec_at (schedule must be set first)
        self.exec_at = self.next_exec_at(clock.unix_timestamp);

        // Increment manager queue counter
        manager.queue_count = manager.queue_count.checked_add(1).unwrap();

        Ok(())
    }

    fn next_exec_at(&self, ts: i64) -> Option<i64> {
        match Schedule::from_str(&self.schedule)
            .unwrap()
            .after(&DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(ts, 0),
                Utc,
            ))
            .take(1)
            .next()
        {
            Some(datetime) => Some(datetime.timestamp()),
            None => None,
        }
    }

    fn roll_forward(&mut self) -> Result<()> {
        self.status = QueueStatus::Pending;
        match self.exec_at {
            Some(exec_at) => self.exec_at = self.next_exec_at(exec_at),
            None => (),
        };
        Ok(())
    }
}

/**
 * QueueStatus
 */

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum QueueStatus {
    Paused,
    Pending,
    Processing { task_id: u128 },
}
