mod components;

use components::navbar::Navbar;
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
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </div>
        </div>
    }
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/files")]
    Files,
    #[at("/streams")]
    Streams,
    #[at("/threads")]
    Threads,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <h1>{"Home"}</h1>},
        Route::Files => html! { <h1>{"Files"}</h1>},
        Route::Streams => html! { <h1>{"Streams"}</h1>},
        Route::Threads => html! { <h1>{"Threads"}</h1>},
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
