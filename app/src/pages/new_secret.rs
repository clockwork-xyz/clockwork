use dioxus::prelude::*;

use super::Page;

pub fn NewSecretPage(cx: Scope) -> Element {
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
                                class: "bg-transparent border-b text-sm font-normal py-2 w-full",
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
                                class: "bg-transparent border-b text-sm font-normal py-2 w-full",
                                r#type: "text",
                                name: "Value"
                            }
                        }
                    }
                    div {
                        class: "flex flex-row w-full justify-between",
                        button {
                            class: "border border-transparent hover:border-white transition py-3 w-full",
                            "Cancel"
                        }
                        button {
                            class: "bg-white py-3 w-full text-slate-800 font-semibold",
                            "Continue"
                        }
                    }
                }
            }
        }
    })
}
