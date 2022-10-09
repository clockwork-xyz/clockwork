use {crate::objects::*, anchor_lang::prelude::*};

#[derive(Accounts)]
pub struct SnapshotDelete<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(address = Registry::pubkey())]
    pub registry: Account<'info, Registry>,

    #[account(address = config.epoch_queue)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            SEED_SNAPSHOT,
            snapshot.id.to_be_bytes().as_ref(),
        ],
        bump,
        constraint = snapshot.id.ne(&registry.current_epoch)
    )]
    pub snapshot: Account<'info, Snapshot>,
}

pub fn handler(ctx: Context<SnapshotDelete>) -> Result<()> {
    // Get accounts
    let snapshot = &mut ctx.accounts.snapshot;
    let signer = &mut ctx.accounts.signer;

    // If this snapshot has no entries, then close immediately
    if snapshot.total_frames == 0 {
        let snapshot_lamports = snapshot.to_account_info().lamports();
        **snapshot.to_account_info().lamports.borrow_mut() = 0;
        **signer.to_account_info().lamports.borrow_mut() = signer
            .to_account_info()
            .lamports()
            .checked_add(snapshot_lamports)
            .unwrap();
    } else {
        // Otherwise, set the status to closing
        // snapshot.status = SnapshotStatus::Closing;
    }

    Ok(())

    // If there are entries to capture, build the next instruction
    // let next_instruction = if snapshot.total_workers > 0 {
    //     let entry_pubkey = SnapshotEntry::pubkey(snapshot.key(), 0);
    //     Some(
    //         Instruction {
    //             program_id: crate::ID,
    //             accounts: vec![
    //                 AccountMeta::new_readonly(authority.key(), false),
    //                 AccountMeta::new(entry_pubkey, false),
    //                 AccountMeta::new(snapshot.key(), false),
    //                 AccountMeta::new(snapshot_queue.key(), true),
    //             ],
    //             data: clockwork_queue_program::utils::anchor_sighash("entry_close").into(),
    //         }
    //         .into(),
    //     )
    // } else {
    //     None
    // };

    // Ok(CrankResponse { next_instruction })
}
