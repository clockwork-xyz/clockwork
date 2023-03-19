use dioxus::prelude::*;

use crate::components::sidebar::Sidebar;

#[derive(Props)]
pub struct PageProps<'a> {
    children: Element<'a>,
}

pub fn Page<'a>(cx: Scope<'a, PageProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            class: "w-full h-full flex flex-row overflow-clip",
            Sidebar {}
            div {
                class: "w-full max-w-7xl pl-8 pt-8 pb-24 pr-8 overflow-y-auto",
                &cx.props.children
            }
        }
    })
}
