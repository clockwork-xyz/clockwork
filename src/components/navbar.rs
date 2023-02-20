use yew::{function_component, html, Callback, Html, MouseEvent, Properties};

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
            <img class="h-4" src="/img/CLOCKWORK.svg" />
        </div>
    }
}
