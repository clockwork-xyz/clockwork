mod components;

use components::navbar::Navbar;
use wasm_logger;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class={classes!("w-screen", "h-screen", "flex", "items-center", "justify-center")}>
            <div class={classes!("flex", "flex-col", "items-center", "justify-center")}>
                <Navbar />
                <h1>{"Hello"}</h1>
            </div>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
