use yew::{function_component, html, prelude::*, Callback, Html, MouseEvent, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub value: String,
    pub onclick: Callback<MouseEvent>,
}

#[function_component]
pub fn Navbar() -> Html {
    html! {
        <div class="fixed top-0 w-screen p-8">
            <Logo />
        </div>
    }
}

#[function_component]
pub fn Logo() -> Html {
    let onclick = { move |_| todo!() };
    html! {
        <button class="p-2" {onclick}>
            <img class="h-4" src="/img/CLOCKWORK.svg" />
        </button>
    }
}
