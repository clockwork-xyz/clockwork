mod button;
mod navbar;

use navbar::Navbar;
use wasm_logger;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="w-screen h-screen flex items-center justify-center">
            <div class="flex flex-col items-center justify-center">
                // <h1 class="p-10 text-xl font-bold">{ "Connect to Phantom Wallet" }</h1>
                // <div id="navbar"><img src="img/CLOCKWORK.svg"/></div>
                // <div class="fixed top-0 w-screen h-8"></div>

                <Navbar />
                <h1>{"Hello"}</h1>
                // <div class=""

            </div>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
