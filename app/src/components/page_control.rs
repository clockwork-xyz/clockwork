use dioxus::prelude::*;

use crate::hooks::UsePagination;

#[inline_props]
pub fn PageControl<T: Clone + 'static>(cx: Scope, paginated_data: UsePagination<T>) -> Element {
    let button_class = "py-2 px-6 text-slate-100 hover:bg-slate-800 active:bg-slate-100 active:text-slate-900 transition text-sm font-medium rounded";
    let hidden_button_class = "py-2 px-6 text-slate-100 hover:bg-slate-800 active:bg-slate-100 active:text-slate-900 transition text-sm font-medium rounded invisible";
    let back_button_class = if paginated_data.current_page().gt(&1) {
        button_class
    } else {
        hidden_button_class
    };

    let forward_button_class = if paginated_data
        .current_page()
        .eq(&paginated_data.total_pages())
    {
        hidden_button_class
    } else {
        button_class
    };

    cx.render(rsx! {
        div {
            class: "flex items-center justify-center space-x-4 mt-2",
            button {
                class: back_button_class,
                onclick: move |_| { paginated_data.prev_page() },
                "←"
            }
            div {
                class: "text-sm text-slate-100",
                "{paginated_data.current_page()} of {paginated_data.total_pages()}"
            }
            button {
                class: forward_button_class,
                onclick: move |_| { paginated_data.next_page() },
                "→"
            }
        }
    })
}
