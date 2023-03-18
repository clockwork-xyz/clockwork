use dioxus::prelude::*;

static API_BASE_URL: &str = "http://3.83.67.25:9000/api";

pub fn HomePage(cx: Scope) -> Element {
    use_future(&cx, (), |_| async move {
        let r = reqwest::Client::new()
            .get(API_BASE_URL)
            .send()
            .await
            .unwrap()
            .text()
            .await;
        log::info!("{:?}", r);
    });

    cx.render(rsx! {
        h1 { "Home" }
    })
}
