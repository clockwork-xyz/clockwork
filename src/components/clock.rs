use chrono::Utc;
use dioxus::prelude::*;

pub fn Clock(cx: Scope) -> Element {
    let time = use_state(&cx, || Utc::now());

    use_future(&cx, (), |_| {
        let time = time.clone();
        async move {
            loop {
                time.set(Utc::now());
                gloo_timers::future::TimeoutFuture::new(1000).await;
            }
        }
    });

    cx.render(rsx! {
        p {
            class: "fixed bottom-0 right-0 p-4",
            time.to_rfc3339()
        }
    })
}
