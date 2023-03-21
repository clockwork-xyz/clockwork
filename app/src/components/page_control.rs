use clockwork_thread_program_v2::state::VersionedThread;
use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::account::Account;

use crate::hooks::UsePagination;

#[derive(Clone, Props)]
pub struct PaginationControlsProps<'a, T> {
    pub paginated_data: &'a UsePagination<T>,
}

impl<'a> PartialEq for PaginationControlsProps<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.paginated_data
            .table_id
            .eq(&other.paginated_data.table_id)
    }
}

pub fn PageControl<'a>(cx: Scope<'a, PaginationControlsProps<'a>>) -> Element<'a> {
    let paginated_data = &cx.props.paginated_data;
    let button_class = "py-2 px-6 text-slate-100 hover:bg-slate-800 active:bg-slate-100 active:text-slate-900 transition text-sm font-medium rounded";
    let hidden_button_class = "py-2 px-6 text-slate-100 hover:bg-slate-800 active:bg-slate-100 active:text-slate-900 transition text-sm font-medium rounded invisible";
    let back_button_class = if paginated_data.current_page().gt(&0) {
        button_class
    } else {
        hidden_button_class
    };
    let forward_button_class = if paginated_data
        .current_page()
        .lt(&(paginated_data.total_pages() - 1))
    {
        button_class
    } else {
        hidden_button_class
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
                "{paginated_data.current_page() + 1} of {paginated_data.total_pages()}"
            }
            button {
                class: forward_button_class,
                onclick: move |_| { paginated_data.next_page() },
                "→"
            }
        }
    })
}
