use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub(crate) enum Route {
    #[at("/")]
    Home,
    #[at("/files")]
    Files,
    #[at("/streams")]
    Streams,
    #[at("/threads")]
    Threads,
}

pub(crate) fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <h1>{"Home"}</h1>},
        Route::Files => html! { <h1>{"Files"}</h1>},
        Route::Streams => html! { <h1>{"Streams"}</h1>},
        Route::Threads => html! { <h1>{"Threads"}</h1>},
    }
}
