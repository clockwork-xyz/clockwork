use cronos_program::state::*;
use solana_program::pubkey::Pubkey;

pub type PDA = (Pubkey, u8);

pub fn find_authority_pda() -> PDA {
    Pubkey::find_program_address(&[SEED_AUTHORITY], &cronos_program::ID)
}

pub fn find_config_pda() -> PDA {
    Pubkey::find_program_address(&[SEED_CONFIG], &cronos_program::ID)
}

pub fn find_daemon_pda(owner: Pubkey) -> PDA {
    Pubkey::find_program_address(&[SEED_DAEMON, owner.as_ref()], &cronos_program::ID)
}

pub fn find_fee_pda(daemon: Pubkey) -> PDA {
    Pubkey::find_program_address(&[SEED_FEE, daemon.as_ref()], &cronos_program::ID)
}

pub fn find_health_pda() -> PDA {
    Pubkey::find_program_address(&[SEED_HEALTH], &cronos_program::ID)
}
pub fn find_task_pda(daemon: Pubkey, id: u128) -> PDA {
    Pubkey::find_program_address(
        &[SEED_TASK, daemon.as_ref(), id.to_be_bytes().as_ref()],
        &cronos_program::ID,
    )
}

pub fn find_treasury_pda() -> PDA {
    Pubkey::find_program_address(&[SEED_TREASURY], &cronos_program::ID)
}
