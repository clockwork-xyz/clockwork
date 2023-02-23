use dioxus::prelude::*;
use dioxus_router::Link;

pub fn Sidebar(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "h-full w-48 flex items-center",
            div {
                class: "w-full flex flex-col my-auto",
                SidebarButton {
                    title: "Data",
                    route: "/data"
                }
                SidebarButton {
                    title: "Files",
                    route: "/files"
                }
                SidebarButton {
                    title: "Threads",
                    route: "/threads"
                }
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
