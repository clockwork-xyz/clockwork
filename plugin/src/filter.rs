use bincode::deserialize;
use cronos_client::scheduler::state::Queue;
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPluginError, ReplicaAccountInfo,
};
use solana_program::{clock::Clock, pubkey::Pubkey, sysvar};

pub enum CronosAccountUpdate {
    Clock { clock: Clock },
    Queue { queue: Queue },
}

impl TryFrom<ReplicaAccountInfo<'_>> for CronosAccountUpdate {
    type Error = GeyserPluginError;
    fn try_from(account_info: ReplicaAccountInfo) -> Result<Self, Self::Error> {
        // If the account is the sysvar clock, return it
        if Pubkey::new(account_info.pubkey).eq(&sysvar::clock::ID) {
            return Ok(CronosAccountUpdate::Clock {
                clock: deserialize::<Clock>(account_info.data).map_err(|_e| {
                    GeyserPluginError::AccountsUpdateError {
                        msg: "Failed to parsed sysvar clock account".into(),
                    }
                })?,
            });
        }

        // If the account is a Cronos queue, return it
        if Pubkey::new(account_info.owner).eq(&cronos_client::scheduler::ID)
            && account_info.data.len() > 8
        {
            return Ok(CronosAccountUpdate::Queue {
                queue: Queue::try_from(account_info.data.to_vec()).map_err(|_| {
                    GeyserPluginError::AccountsUpdateError {
                        msg: "Failed to parse cronos queue account".into(),
                    }
                })?,
            });
        }

        Err(GeyserPluginError::AccountsUpdateError {
            msg: "Account is not relevant to cronos plugin".into(),
        })
    }
}
