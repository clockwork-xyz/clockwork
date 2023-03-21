use anchor_lang::prelude::Pubkey;
use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::account::Account;

use crate::utils::format_balance;

#[derive(PartialEq, Props)]
pub struct AccountInfoTableProps {
    account: Account,
    address: Pubkey,
}

pub fn AccountInfoTable(cx: Scope<AccountInfoTableProps>) -> Element {
    let account = &cx.props.account;
    let address = &cx.props.address;
    let balance = format_balance(account.lamports, false);
    let executable = account.executable;
    let owner = account.owner;
    cx.render(rsx! {
        table {
            class: "w-full divide-y divide-slate-800",
            tbody {
                Row {
                    label: "Address".to_string(),
                    value: address.to_string()
                }
                Row {
                    label: "Balance".to_string(),
                    value: balance,
                }
                Row {
                    label: "Executable".to_string(),
                    value: executable.to_string(),
                }
                Row {
                    label: "Owner".to_string(),
                    value: owner.to_string(),
                }
            }
        }
    })
}

#[derive(PartialEq, Props)]
struct RowProps {
    label: String,
    value: String,
}

fn Row(cx: Scope<RowProps>) -> Element {
    cx.render(rsx! {
        div {
            class: "flex justify-between",
            id: cx.props.label.as_str(),
            div {
                class: "table-cell whitespace-nowrap px-2 py-2 text-sm text-slate-500",
                "{cx.props.label}"
            }
            div {
                class: "table-cell whitespace-nowrap px-2 py-2 text-sm font-mono text-slate-100",
                "{cx.props.value}"
            }
        }
    })
}
