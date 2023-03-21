use dioxus::prelude::*;
use dioxus_router::{use_route, Link};

#[derive(Clone, PartialEq)]
pub enum SidebarOption {
    Accounts,
    Files,
    Keys,
    Programs,
}

impl SidebarOption {
    pub fn title(self: Self) -> &'static str {
        match self {
            SidebarOption::Accounts => "Accounts",
            SidebarOption::Files => "Files",
            SidebarOption::Keys => "Keys",
            SidebarOption::Programs => "Programs",
        }
    }
    pub fn route(self: Self) -> &'static str {
        match self {
            SidebarOption::Accounts => "/accounts",
            SidebarOption::Files => "/files",
            SidebarOption::Keys => "/keys",
            SidebarOption::Programs => "/programs",
        }
    }
    pub fn icon(self: Self, cx: Scope<SidebarButtonProps>) -> Element {
        cx.render(rsx! {
            match self {
                SidebarOption::Accounts => rsx! { AccountsIcon {} },
                SidebarOption::Files => rsx! { FilesIcon {} },
                SidebarOption::Keys => rsx! { KeysIcon {} },
                SidebarOption::Programs => rsx! { ProgramsIcon {} },
            }
        })
    }
}

pub fn Sidebar(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "h-full w-48 flex flex-col items-start py-8 pl-4",
            SidebarButton {
                option: SidebarOption::Accounts
            }
            SidebarButton {
                option: SidebarOption::Files
            }
            SidebarButton {
                option: SidebarOption::Keys
            }
            SidebarButton {
                option: SidebarOption::Programs
            }
        }
    })
}

#[derive(PartialEq, Props)]
pub struct SidebarButtonProps {
    option: SidebarOption,
}

pub fn SidebarButton(cx: Scope<SidebarButtonProps>) -> Element {
    let option = &cx.props.option;
    let title = option.clone().title();
    let route = option.clone().route();
    let icon = option.clone().icon(cx);

    let is_selected = use_route(&cx)
        .nth_segment(0)
        .eq(&Some(route.trim_start_matches("/")));

    cx.render(rsx! {
        Link {
            to: route,
            class: if is_selected {
                "w-40 px-4 py-3 text-left rounded transition text-slate-100 hover:bg-slate-800 active:bg-slate-100 active:text-slate-900 font-medium"
            } else {
                "w-40 px-4 py-3 text-left rounded transition text-slate-500 hover:bg-slate-800 active:bg-slate-100 active:text-slate-900 font-medium"
            },
            div {
                class: "flex flex-row items-start gap-3 w-full",
                icon
                "{title}"
            }
        }
    })
}

pub fn AccountsIcon(cx: Scope) -> Element {
    cx.render(rsx! {
        svg {
            class: "w-6 h-6 my-auto",
            fill: "none",
            view_box: "0 0 24 24",
            stroke_width: "1.5",
            stroke: "currentColor",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M21 7.5l-9-5.25L3 7.5m18 0l-9 5.25m9-5.25v9l-9 5.25M3 7.5l9 5.25M3 7.5v9l9 5.25m0-9v9"
            }
        }
    })
}

pub fn KeysIcon(cx: Scope) -> Element {
    cx.render(rsx! {
        svg {
            class: "w-6 h-6 my-auto",
            fill: "none",
            view_box: "0 0 24 24",
            stroke_width: "1.5",
            stroke: "currentColor",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M15.75 5.25a3 3 0 013 3m3 0a6 6 0 01-7.029 5.912c-.563-.097-1.159.026-1.563.43L10.5 17.25H8.25v2.25H6v2.25H2.25v-2.818c0-.597.237-1.17.659-1.591l6.499-6.499c.404-.404.527-1 .43-1.563A6 6 0 1121.75 8.25z"
            }
        }
    })
}

pub fn FilesIcon(cx: Scope) -> Element {
    cx.render(rsx! {
        svg {
            class: "w-6 h-6 my-auto",
            fill: "none",
            view_box: "0 0 24 24",
            stroke_width: "1.5",
            stroke: "currentColor",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M2.25 12.75V12A2.25 2.25 0 014.5 9.75h15A2.25 2.25 0 0121.75 12v.75m-8.69-6.44l-2.12-2.12a1.5 1.5 0 00-1.061-.44H4.5A2.25 2.25 0 002.25 6v12a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18V9a2.25 2.25 0 00-2.25-2.25h-5.379a1.5 1.5 0 01-1.06-.44z"
            }
        }
    })
}

pub fn ProgramsIcon(cx: Scope) -> Element {
    cx.render(rsx! {
        svg {
            class: "w-6 h-6 my-auto",
            fill: "none",
            view_box: "0 0 24 24",
            stroke_width: "1.5",
            stroke: "currentColor",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M6.75 7.5l3 2.25-3 2.25m4.5 0h3m-9 8.25h13.5A2.25 2.25 0 0021 18V6a2.25 2.25 0 00-2.25-2.25H5.25A2.25 2.25 0 003 6v12a2.25 2.25 0 002.25 2.25z"
            }
        }
    })
}
