use dioxus::prelude::*;
use dioxus_router::{use_route, Link};

pub fn Sidebar(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "h-full w-48 flex flex-col items-start py-8",
                SidebarButton {
                    title: "Data",
                    route: "/data"
                }
                SidebarButton {
                    title: "Files",
                    route: "/files"
                }
                SidebarButton {
                    title: "Programs",
                    route: "/programs"
                }
                SidebarButton {
                    title: "Secrets",
                    route: "/secrets"
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
    let is_selected = use_route(&cx)
        .nth_segment(0)
        .eq(&Some(cx.props.route.trim_start_matches("/")));

    cx.render(rsx! {
        Link {
            to: cx.props.route,
            class: if is_selected {
                "w-full py-2 px-8 flex flex-row text-left hover:bg-slate-900"
            } else {
                "w-full py-2 px-8 flex flex-row text-left text-slate-600 hover:bg-slate-900"
            },
            "{cx.props.title}"
        }
    })
}
