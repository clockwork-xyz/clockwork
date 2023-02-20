use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component]
pub fn Sidebar() -> Html {
    html! {
        <div class={classes!("fixed", "left-0", "flex", "flex-col", "px-8")}>
            <SidebarButton title={"Data"} route={Route::Data} />
            <SidebarButton title={"Files"} route={Route::Files} />
            <SidebarButton title={"Threads"} route={Route::Threads} />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct SidebarButtonProps {
    #[prop_or_default]
    pub title: AttrValue,
    pub route: Route,
}

#[function_component]
pub fn SidebarButton(props: &SidebarButtonProps) -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = {
        let route = props.route.clone();
        move |_| navigator.push(&route)
    };
    html! {
        <button class={classes!("p-2", "w-full", "text-left")} {onclick}>
            <p>{props.title.clone()}</p>
        </button>
    }
}
