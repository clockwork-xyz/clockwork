use solana_geyser_plugin_interface::geyser_plugin_interface::ReplicaAccountInfo;
use solana_program::{pubkey::Pubkey, sysvar};

pub fn wants_account(info: &ReplicaAccountInfo) -> bool {
    // If the account is the sysvar clock, return true
    let account_pubkey = Pubkey::new(info.pubkey);
    if account_pubkey.eq(&sysvar::clock::ID) {
        return true;
    }

    // If the account is a cronos queue, return true
    if info.data.len() > 8 {
        let owner_pubkey = Pubkey::new(info.owner);
        if owner_pubkey == cronos_sdk::scheduler::ID {
            return true;
        }
    }

    // Ignore everything else
    return false;
}
