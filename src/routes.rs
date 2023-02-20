use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/data")]
    Data,
    #[at("/files")]
    Files,
    #[at("/threads")]
    Threads,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <h1>{"Home"}</h1>},
        Route::Data => html! { <h1>{"Data"}</h1>},
        Route::Files => html! { <h1>{"Files"}</h1>},
        Route::Threads => html! { <h1>{"Threads"}</h1>},
    }
}
