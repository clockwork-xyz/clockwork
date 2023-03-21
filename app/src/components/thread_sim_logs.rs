use std::convert::From;

use dioxus::prelude::*;
use clockwork_thread_program_v2::state::VersionedThread;
use solana_client_wasm::solana_sdk::transaction::TransactionError;

use crate::clockwork::simulate_thread;

#[derive(PartialEq, Props)]
pub struct ThreadSimLogsProps{
    thread: VersionedThread,
}

pub fn ThreadSimLogs(cx: Scope<ThreadSimLogsProps>) -> Element {
    let sim_logs = use_state::<Vec<String>>(cx, || vec![]);
    let sim_error = use_state::<Option<String>>(cx, || None);

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

