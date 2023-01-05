use {
    crate::state::*,
    anchor_lang::prelude::*,
    clockwork_utils::{anchor_sighash, AccountMetaData, InstructionData, ThreadResponse},
};

#[derive(Accounts)]
pub struct DeleteSnapshotJob<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(
        address = Registry::pubkey(),
        constraint = !registry.locked
    )]
    pub registry: Account<'info, Registry>,

    #[account(address = config.epoch_thread)]
    pub thread: Signer<'info>,
}

pub fn handler(ctx: Context<DeleteSnapshotJob>) -> Result<ThreadResponse> {
    let config = &ctx.accounts.config;
    let registry = &ctx.accounts.registry;
    let thread = &mut ctx.accounts.thread;

    Ok(ThreadResponse {
        next_instruction: Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                AccountMetaData::new_readonly(config.key(), false),
                AccountMetaData::new_readonly(registry.key(), false),
                AccountMetaData::new(
                    Snapshot::pubkey(registry.current_epoch.checked_sub(1).unwrap()),
                    false,
                ),
                AccountMetaData::new(thread.key(), true),
            ],
            data: anchor_sighash("delete_snapshot_process_snapshot").to_vec(),
        }),
    })
}
