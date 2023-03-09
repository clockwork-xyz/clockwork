use dioxus::prelude::*;
use dioxus_router::use_router;

use super::Page;

pub fn NewSecretPage(cx: Scope) -> Element {
    let router = use_router(&cx);

    cx.render(rsx! {
        Page {
            div {
                class: "flex justify-around h-full w-full",
                div {
                    class: "flex flex-col m-auto w-full max-w-3xl space-y-6",
                    h1 {
                        class: "text-2xl font-semibold",
                        "New Secret"
                    }
                    form {
                        class: "flex flex-col space-y-8 w-full",
                        div {
                            p {
                                class: "text-left text-sm font-medium mb-1",
                                "Name"
                            }
                            input {
                                class: "bg-transparent border-b text-base font-normal py-3 px-3 w-full hover:bg-slate-100 hover:text-slate-900",
                                r#type: "text",
                                name: "Name"
                            }
                        }
                        div {
                            p {
                                class: "text-left text-sm font-medium mb-1",
                                "Value"
                            }
                            input {
                                class: "bg-transparent border-b text-base font-normal py-3 px-3 w-full hover:bg-slate-100 hover:text-slate-900",
                                r#type: "text",
                                name: "Value"
                            }
                        }
                    }
                    div {
                        class: "flex flex-row w-full justify-between",
                        button {
                            class: "font-normal text-slate-100 bg-transparent hover:bg-slate-100 hover:text-slate-900 transition py-3 w-full",
                            onclick: move |_| { router.pop_route() },
                            "Cancel"
                        }
                        button {
                            class: "font-semibold text-slate-100 bg-transparent hover:bg-slate-100 hover:text-slate-900 transition py-3 w-full",
                            onclick: move |_| {
                                // TODO
                            },
                            "Continue"
                        }
                    }
                }
            }
        }
    })
}
