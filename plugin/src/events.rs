use anchor_lang::{prelude::AccountInfo, AccountDeserialize, Discriminator};
use bincode::deserialize;
use clockwork_thread_program::state::{Thread as ThreadV2, VersionedThread};
use clockwork_thread_program_v1::state::Thread as ThreadV1;
use clockwork_webhook_program::state::Webhook;
use pyth_sdk_solana::{load_price_feed_from_account_info, PriceFeed};
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPluginError, ReplicaAccountInfo,
};
use solana_program::{clock::Clock, pubkey::Pubkey, sysvar};
use static_pubkey::static_pubkey;

static PYTH_ORACLE_PROGRAM_ID_MAINNET: Pubkey =
    static_pubkey!("FsJ3A3u2vn5cTVofAjvy6y5kwABJAqYWpe4975bi2epH");
static PYTH_ORACLE_PROGRAM_ID_DEVNET: Pubkey =
    static_pubkey!("gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s");

#[derive(Debug)]
pub enum AccountUpdateEvent {
    Clock { clock: Clock },
    Thread { thread: VersionedThread },
    PriceFeed { price_feed: PriceFeed },
    Webhook { webhook: Webhook },
}

impl TryFrom<&mut ReplicaAccountInfo<'_>> for AccountUpdateEvent {
    type Error = GeyserPluginError;
    fn try_from(account_info: &mut ReplicaAccountInfo) -> Result<Self, Self::Error> {
        // Parse pubkeys.
        let account_pubkey = Pubkey::try_from(account_info.pubkey).unwrap();
        let owner_pubkey = Pubkey::try_from(account_info.owner).unwrap();

        // If the account is the sysvar clock, parse it.
        if account_pubkey.eq(&sysvar::clock::ID) {
            return Ok(AccountUpdateEvent::Clock {
                clock: deserialize::<Clock>(account_info.data).map_err(|_e| {
                    GeyserPluginError::AccountsUpdateError {
                        msg: "Failed to parsed sysvar clock account".into(),
                    }
                })?,
            });
        }

        // If the account belongs to the thread v1 program, parse it.
        if owner_pubkey.eq(&clockwork_thread_program_v1::ID) && account_info.data.len() > 8 {
            let d = &account_info.data[..8];
            if d.eq(&ThreadV1::discriminator()) {
                return Ok(AccountUpdateEvent::Thread {
                    thread: VersionedThread::V1(
                        ThreadV1::try_deserialize(&mut account_info.data).map_err(|_| {
                            GeyserPluginError::AccountsUpdateError {
                                msg: "Failed to parse Clockwork thread v1 account".into(),
                            }
                        })?,
                    ),
                });
            }
        }

        // If the account belongs to the thread v2 program, parse it.
        if owner_pubkey.eq(&clockwork_thread_program::ID) && account_info.data.len() > 8 {
            let d = &account_info.data[..8];
            if d.eq(&ThreadV2::discriminator()) {
                return Ok(AccountUpdateEvent::Thread {
                    thread: VersionedThread::V2(
                        ThreadV2::try_deserialize(&mut account_info.data).map_err(|_| {
                            GeyserPluginError::AccountsUpdateError {
                                msg: "Failed to parse Clockwork thread v2 account".into(),
                            }
                        })?,
                    ),
                });
            }
        }

        // If the account belongs to Pyth, attempt to parse it.
        if owner_pubkey.eq(&PYTH_ORACLE_PROGRAM_ID_MAINNET)
            || owner_pubkey.eq(&PYTH_ORACLE_PROGRAM_ID_DEVNET)
        {
            let data = &mut account_info.data.to_vec();
            let acc_info = AccountInfo::new(
                &account_pubkey,
                false,
                false,
                &mut account_info.lamports,
                data,
                &owner_pubkey,
                account_info.executable,
                account_info.rent_epoch,
            );
            let price_feed = load_price_feed_from_account_info(&acc_info).map_err(|_| {
                GeyserPluginError::AccountsUpdateError {
                    msg: "Failed to parse Pyth price account".into(),
                }
            })?;
            return Ok(AccountUpdateEvent::PriceFeed { price_feed });
        }

        // If the account belongs to the webhook program, parse in
        if owner_pubkey.eq(&clockwork_webhook_program::ID) && account_info.data.len() > 8 {
            return Ok(AccountUpdateEvent::Webhook {
                webhook: Webhook::try_deserialize(&mut account_info.data).map_err(|_| {
                    GeyserPluginError::AccountsUpdateError {
                        msg: "Failed to parse Clockwork webhook".into(),
                    }
                })?,
            });
        }

        Err(GeyserPluginError::AccountsUpdateError {
            msg: "Account is not relevant to Clockwork plugin".into(),
        })
    }
}
