use std::str::FromStr;
use std::convert::From;
use anchor_lang::solana_program::pubkey::Pubkey; 
use chrono::{DateTime, NaiveDateTime, Utc};
use dioxus::prelude::*;
use dioxus_router::use_route;
use clockwork_thread_program_v2::state::{VersionedThread, Trigger};
use solana_client_wasm::solana_sdk::{transaction::TransactionError, account::Account};

use crate::{clockwork::{get_thread, simulate_thread}, utils::{format_timestamp, format_balance}};

use super::Page;

pub fn ThreadPage(cx: Scope) -> Element {
    let route = use_route(cx);
    let thread = use_state::<Option<(VersionedThread, Account)>>(cx, || None);

    use_future(&cx, (), |_| {
        let thread = thread.clone();
        let thread_pubkey = Pubkey::from_str(route.last_segment().unwrap()).unwrap();
        async move { 
            let t = get_thread(thread_pubkey).await;
            thread.set(Some(t.clone()));
        }
    });
    
    if let Some(t) = thread.get() {
        cx.render(rsx! {
            Page {
                div {
                    class: "flex flex-col space-y-16",
                    div {
                        class: "flex flex-col justify-between",
                        h1 {
                             class: "text-2xl font-semibold mb-6",
                             "Thread"
                        }
                        ThreadInfoTable { account: t.clone().1, thread: t.clone().0 }
                    }
                    SimulationLogs { thread: t.clone().0 }
                }
            }
        })
    } else {
        cx.render(rsx! {
            Page {
                div {
                    "Loading..."
                }
            }
        })
    }
}

#[derive(PartialEq, Props)]
struct ThreadInfoTableProps{
    account: Account,
    thread: VersionedThread,
}

fn ThreadInfoTable(cx: Scope<ThreadInfoTableProps>) -> Element {
    let thread = &cx.props.thread;
    let address = thread.pubkey();
    let balance = format_balance(cx.props.account.lamports, false);
    let created_at = format_timestamp(thread.created_at().unix_timestamp);
    let trigger = match thread.trigger().clone() {
        Trigger::Account {
            address,
            offset: _,
            size: _,
        } => address.to_string(),
        Trigger::Cron {
            schedule,
            skippable: _,
        } => schedule.clone(),
        Trigger::Now => "Now".to_string(),
        Trigger::Slot { slot } => slot.to_string(),
        Trigger::Epoch { epoch } => epoch.to_string(),
        Trigger::Timestamp { unix_ts } => unix_ts.to_string()
    };

    let id = String::from_utf8( thread.id()).unwrap();

    cx.render(rsx! {
        table {
            class: "w-full divide-y divide-slate-800",
            tbody {
                Row {
                    label: "Address".to_string(),
                    value: address.to_string()
                }
                Row {
                    label: "Authority".to_string(),
                    value: thread.authority().to_string(),
                }
                Row {
                    label: "Balance".to_string(),
                    value: balance,
                }
                Row {
                    label: "Created at".to_string(),
                    value: created_at,
                }
                // Row {
                //     label: "Fee".to_string(),
                //     value: thread.fee.to_string(),
                // }
                Row {
                    label: "ID".to_string(),
                    value: id,
                }
                Row {
                    label: "Paused".to_string(),
                    value: thread.paused().to_string(),
                }
                Row {
                    label: "Trigger".to_string(),
                    value: trigger,
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

#[derive(PartialEq, Props)]
struct SimulationLogsProps{
    thread: VersionedThread,
}


fn SimulationLogs(cx: Scope<SimulationLogsProps>) -> Element {
    let sim_logs = use_state::<Vec<String>>(cx, || vec![]);
    let sim_error = use_state::<Option<String>>(cx, || None);

    // let tx_err = if sim_error.get().is_some() {
    //     let mut hex: Option<String> = None;
    //     let raw_text = sim_error.as_ref().unwrap().to_string();
    //     let mut error_string = String::new();
    //     for word in raw_text.split(" ").collect::<Vec<_>>().iter() {
    //         hex = match word.strip_prefix("0x") {
    //             Some(d) => Some(String::from(d.to_string())),
    //             None => {
    //                 let w = String::from(format!("{} ", word));
    //                 error_string.push_str(w.as_str());
    //                 None
    //             },
    //         }
    //     }

    //     if hex.is_some() {
    //         let dec = i64::from_str_radix(hex.unwrap().as_str(), 16);
    //         String::from(format!("{}{}", error_string, dec.unwrap()))
    //     } else {
    //         String::from(format!("{}", error_string))
    //     }
        
    // } else {
    //     String::from("")
    // };

    use_future(&cx, (), |_| {
        let thread = cx.props.thread.clone();
        let sim_logs = sim_logs.clone();
        let sim_error = sim_error.clone();
        async move { 
            match simulate_thread(thread.to_owned(), thread.pubkey()).await {
                Ok((err, logs)) => {
                    sim_logs.set(logs.unwrap_or(vec![]));
                    let err_msg = if let Some(err) = err {
                        match err {
                            // TransactionError::AccountInUse => todo!(),
                            // TransactionError::AccountLoadedTwice => todo!(),
                            // TransactionError::AccountNotFound => todo!(),
                            // TransactionError::ProgramAccountNotFound => todo!(),
                            // TransactionError::InsufficientFundsForFee => todo!(),
                            // TransactionError::InvalidAccountForFee => todo!(),
                            // TransactionError::AlreadyProcessed => todo!(),
                            // TransactionError::BlockhashNotFound => todo!(),
                            TransactionError::InstructionError(_, ix_err) => {
                                match ix_err {
                                    // anchor_lang::solana_program::instruction::InstructionError::GenericError => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::InvalidArgument => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::InvalidInstructionData => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::InvalidAccountData => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::AccountDataTooSmall => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::InsufficientFunds => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::IncorrectProgramId => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::MissingRequiredSignature => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::AccountAlreadyInitialized => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::UninitializedAccount => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::UnbalancedInstruction => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::ModifiedProgramId => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::ExternalAccountLamportSpend => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::ExternalAccountDataModified => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::ReadonlyLamportChange => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::ReadonlyDataModified => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::DuplicateAccountIndex => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::ExecutableModified => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::RentEpochModified => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::NotEnoughAccountKeys => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::AccountDataSizeChanged => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::AccountNotExecutable => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::AccountBorrowFailed => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::AccountBorrowOutstanding => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::DuplicateAccountOutOfSync => todo!(),
                                    anchor_lang::solana_program::instruction::InstructionError::Custom(err_code) => {
                                        if err_code.eq(&u32::from(clockwork_thread_program_v2::errors::ClockworkError::TriggerConditionFailed)) {
                                            // TODO Let the user know this thread is waiting to be triggered.
                                            None
                                        } else {
                                            Some(ix_err.to_string())
                                        }
                                    },
                                    // anchor_lang::solana_program::instruction::InstructionError::InvalidError => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::ExecutableDataModified => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::ExecutableLamportChange => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::ExecutableAccountNotRentExempt => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::UnsupportedProgramId => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::CallDepth => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::MissingAccount => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::ReentrancyNotAllowed => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::MaxSeedLengthExceeded => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::InvalidSeeds => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::InvalidRealloc => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::ComputationalBudgetExceeded => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::PrivilegeEscalation => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::ProgramEnvironmentSetupFailure => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::ProgramFailedToComplete => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::ProgramFailedToCompile => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::Immutable => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::IncorrectAuthority => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::BorshIoError(_) => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::AccountNotRentExempt => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::InvalidAccountOwner => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::ArithmeticOverflow => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::UnsupportedSysvar => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::IllegalOwner => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::MaxAccountsDataSizeExceeded => todo!(),
                                    // anchor_lang::solana_program::instruction::InstructionError::MaxAccountsExceeded => todo!(),
                                    _ => Some(ix_err.to_string())
                                }
                                // Some(err.to_string())
                                // Some(String::from("whats"))
                            },
                            // TransactionError::CallChainTooDeep => todo!(),
                            // TransactionError::MissingSignatureForFee => todo!(),
                            // TransactionError::InvalidAccountIndex => todo!(),
                            // TransactionError::SignatureFailure => todo!(),
                            // TransactionError::InvalidProgramForExecution => todo!(),
                            // TransactionError::SanitizeFailure => todo!(),
                            // TransactionError::ClusterMaintenance => todo!(),
                            // TransactionError::AccountBorrowOutstanding => todo!(),
                            // TransactionError::WouldExceedMaxBlockCostLimit => todo!(),
                            // TransactionError::UnsupportedVersion => todo!(),
                            // TransactionError::InvalidWritableAccount => todo!(),
                            // TransactionError::WouldExceedMaxAccountCostLimit => todo!(),
                            // TransactionError::WouldExceedAccountDataBlockLimit => todo!(),
                            // TransactionError::TooManyAccountLocks => todo!(),
                            // TransactionError::AddressLookupTableNotFound => todo!(),
                            // TransactionError::InvalidAddressLookupTableOwner => todo!(),
                            // TransactionError::InvalidAddressLookupTableData => todo!(),
                            // TransactionError::InvalidAddressLookupTableIndex => todo!(),
                            // TransactionError::InvalidRentPayingAccount => todo!(),
                            // TransactionError::WouldExceedMaxVoteCostLimit => todo!(),
                            // TransactionError::WouldExceedAccountDataTotalLimit => todo!(),
                            // TransactionError::DuplicateInstruction(_) => todo!(),
                            TransactionError::InsufficientFundsForRent { account_index } => {
                                if account_index.eq(&1) {
                                    // Some("Please fund the thread account".to_string())
                                    // TODO Let the user know they need to fund the thread account.
                                    Some(err.to_string())
                                } else {
                                    Some(err.to_string())
                                }
                            },
                            _ => Some(err.to_string())
                        }
                    } else {
                            None
                        };
                    sim_error.set(err_msg);
                },
                Err(_err) => {}
            }
        }
    });

    cx.render(rsx! {
        div {
            class: "flex flex-col",
            h1 {
                class: "text-2xl text-slate-100 font-semibold font-header mb-4",
                "Simulation Logs"
            }
            if let Some(err) = sim_error.get() {
                rsx! {
                    div {
                        class: "w-full h-auto flex flex-col p-4 space-y-2 break-all rounded bg-red-500",
                        p {
                            class: "text-slate-100 text-sm",
                            "ERROR"
                        }
                        p {
                            class: "text-slate-100 text-base",
                            "{err}"
                        }
                    }
                }
            }
            code {
                class: "w-full h-auto flex flex-col px-4 py-2 font-mono text-base text-slate-100 break-all",
                for log in sim_logs.get().iter() {
                    p {
                        "{log}"
                    }   
                }
            }
        }
    })
}

