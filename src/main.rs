mod components;
mod routes;

use components::*;
use routes::*;
use wasm_logger;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class={classes!("w-screen", "h-screen", "flex", "items-center", "justify-center")}>
            <div class={classes!("flex", "flex-col", "items-center", "justify-center")}>
                <BrowserRouter>
                    <Navbar />
                    <Sidebar />
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </div>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
