use dioxus::prelude::*;
use dioxus_router::Link;

use crate::routes::RoutePath;

pub fn Sidebar(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "fixed left-0 flex flex-col px-8",
            SidebarButton {
                title: "Data",
                route: RoutePath::Data.as_str()
            }
            SidebarButton {
                title: "Files",
                route: RoutePath::Files.as_str()
            }
            SidebarButton {
                title: "Threads",
                route: RoutePath::Threads.as_str()
            }
        }
    })
}

#[derive(PartialEq, Props)]
pub struct SidebarButtonProps<'a> {
    title: &'a str,
    route: &'a str,
}

pub fn SidebarButton<'a>(cx: Scope<'a, SidebarButtonProps<'a>>) -> Element {
    cx.render(rsx! {
        Link {
            to: cx.props.route,
            class: "w-full p-2 text-left",
            "{cx.props.title}"
        }
    })
}
