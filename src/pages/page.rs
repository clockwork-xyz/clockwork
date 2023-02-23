use dioxus::prelude::*;

use crate::components::sidebar::Sidebar;

#[derive(Props)]
pub struct PageProps<'a> {
    children: Element<'a>,
}

pub fn Page<'a>(cx: Scope<'a, PageProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            class: "w-full h-full flex flex-row",
            Sidebar {}
            div {
                class: "w-full h-full p-8 pr-12",
                &cx.props.children
            }
        }
    })
}
