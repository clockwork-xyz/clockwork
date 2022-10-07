use {
    anchor_lang::Discriminator,
    bincode::deserialize,
    clockwork_client::{
        network::objects::{Pool, Registry, Rotator},
        queue::objects::Queue,
        webhook::objects::Request,
    },
    solana_geyser_plugin_interface::geyser_plugin_interface::{
        GeyserPluginError, ReplicaAccountInfo,
    },
    solana_program::{clock::Clock, pubkey::Pubkey, sysvar},
};

pub enum AccountUpdateEvent {
    Clock { clock: Clock },
    HttpRequest { request: Request },
    Pool { pool: Pool },
    Queue { queue: Queue },
    Registry { registry: Registry },
    Rotator { rotator: Rotator },
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

        if owner_pubkey.eq(&clockwork_client::network::ID) && account_info.data.len() > 8 {
            let d = &account_info.data[..8];
            if d.eq(&Pool::discriminator()) {
                return Ok(AccountUpdateEvent::Pool {
                    pool: Pool::try_from(account_info.data.to_vec()).map_err(|_| {
                        GeyserPluginError::AccountsUpdateError {
                            msg: "Failed to parse Clockwork pool account".into(),
                        }
                    })?,
                });
            } else if d.eq(&Registry::discriminator()) {
                return Ok(AccountUpdateEvent::Registry {
                    registry: Registry::try_from(account_info.data.to_vec()).map_err(|_| {
                        GeyserPluginError::AccountsUpdateError {
                            msg: "Failed to parse Clockwork registry account".into(),
                        }
                    })?,
                });
            } else if d.eq(&Rotator::discriminator()) {
                // If the account is the rotator, return it
                return Ok(AccountUpdateEvent::Rotator {
                    rotator: Rotator::try_from(account_info.data.to_vec()).map_err(|_| {
                        GeyserPluginError::AccountsUpdateError {
                            msg: "Failed to parse Clockwork rotator account".into(),
                        }
                    })?,
                });
            }
        }

        if owner_pubkey.eq(&clockwork_client::queue::ID) && account_info.data.len() > 8 {
            let d = &account_info.data[..8];
            if d.eq(&Queue::discriminator()) {
                // If the account is a queue, return it
                return Ok(AccountUpdateEvent::Queue {
                    queue: Queue::try_from(account_info.data.to_vec()).map_err(|_| {
                        GeyserPluginError::AccountsUpdateError {
                            msg: "Failed to parse Clockwork queue account".into(),
                        }
                    })?,
                });
            }
        }

        // If the account is an webhook request, return in
        if owner_pubkey.eq(&clockwork_client::webhook::ID) && account_info.data.len() > 8 {
            return Ok(AccountUpdateEvent::HttpRequest {
                request: Request::try_from(account_info.data.to_vec()).map_err(|_| {
                    GeyserPluginError::AccountsUpdateError {
                        msg: "Failed to parse Clockwork http request".into(),
                    }
                })?,
            });
        }

        Err(GeyserPluginError::AccountsUpdateError {
            msg: "Account is not relevant to Clockwork plugin".into(),
        })
    }
}
