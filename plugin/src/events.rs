use bincode::deserialize;
use cached::proc_macro::cached;
use cronos_client::{
    network::state::{Cycler, Snapshot},
    pool::state::Pool,
    scheduler::state::Queue,
};
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPluginError, ReplicaAccountInfo,
};
use solana_program::{account_info::Account, clock::Clock, pubkey::Pubkey, sysvar};

pub enum AccountUpdateEvent {
    Clock { clock: Clock },
    Cycler { cycler: Cycler },
    Pool { pool: Pool },
    Queue { queue: Queue },
    Snapshot { snapshot: Snapshot },
}

impl TryFrom<ReplicaAccountInfo<'_>> for AccountUpdateEvent {
    type Error = GeyserPluginError;
    fn try_from(account_info: ReplicaAccountInfo) -> Result<Self, Self::Error> {
        let account_pubkey = Pubkey::new(account_info.pubkey);
        let owner_pubkey = Pubkey::new(account_info.owner);

        // If the account is the sysvar clock, return it
        if account_pubkey.eq(&sysvar::clock::ID) {
            return Ok(AccountUpdateEvent::Clock {
                clock: deserialize::<Clock>(account_info.data).map_err(|_e| {
                    GeyserPluginError::AccountsUpdateError {
                        msg: "Failed to parsed sysvar clock account".into(),
                    }
                })?,
            });
        }

        // If the account is the Cronos cycler, return it
        if account_pubkey.eq(&cycler_pubkey()) {
            return Ok(AccountUpdateEvent::Cycler {
                cycler: Cycler::try_from(account_info.data.to_vec()).map_err(|_| {
                    GeyserPluginError::AccountsUpdateError {
                        msg: "Failed to parse Cronos cycler account".into(),
                    }
                })?,
            });
        }

        // If the account is the Cronos delegate pool, return it
        if account_pubkey.eq(&pool_pubkey()) {
            return Ok(AccountUpdateEvent::Pool {
                pool: Pool::try_from(account_info.data.to_vec()).map_err(|_| {
                    GeyserPluginError::AccountsUpdateError {
                        msg: "Failed to parse Cronos pool account".into(),
                    }
                })?,
            });
        }

        // If the account is a Cronos queue, return it
        if owner_pubkey.eq(&cronos_client::scheduler::ID) && account_info.data.len() > 8 {
            return Ok(AccountUpdateEvent::Queue {
                queue: Queue::try_from(account_info.data.to_vec()).map_err(|_| {
                    GeyserPluginError::AccountsUpdateError {
                        msg: "Failed to parse Cronos queue account".into(),
                    }
                })?,
            });
        }

        if owner_pubkey.eq(&cronos_client::network::ID) && account_info.data.len() > 8 {
            return Ok(AccountUpdateEvent::Snapshot {
                snapshot: Snapshot::try_from(account_info.data.to_vec()).map_err(|_| {
                    GeyserPluginError::AccountsUpdateError {
                        msg: "Failed to parse Cronos snapshot account".into(),
                    }
                })?,
            });
        }

        Err(GeyserPluginError::AccountsUpdateError {
            msg: "Account is not relevant to cronos plugin".into(),
        })
    }
}

#[cached]
fn cycler_pubkey() -> Pubkey {
    Cycler::pda().0
}

#[cached]
fn pool_pubkey() -> Pubkey {
    Pool::pda().0
}
