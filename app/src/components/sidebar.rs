use dioxus::prelude::*;
use dioxus_router::{use_route, Link};

pub fn Sidebar(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "h-full w-48 flex flex-col items-start py-8",
                SidebarButton {
                    title: "Accounts",
                    route: "/accounts"
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
                "w-full px-8 py-3 text-left text-slate-100 hover:bg-slate-100 hover:text-slate-900 font-medium"
            } else {
                "w-full px-8 py-3 text-left text-slate-500 hover:bg-slate-100 hover:text-slate-900 font-medium"
            },
            "{cx.props.title}"
        }
    })
}
