use {crate::objects::*, anchor_lang::prelude::*};

#[derive(Accounts)]
pub struct SnapshotClose<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(address = config.authorized_queue)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            SEED_SNAPSHOT,
            snapshot.epoch.as_ref(),
        ],
        bump,
        constraint = snapshot.status == SnapshotStatus::Archived
    )]
    pub snapshot: Account<'info, Snapshot>,
}

pub fn handler(ctx: Context<SnapshotClose>) -> Result<()> {
    // Get accounts
    let snapshot = &mut ctx.accounts.snapshot;
    let signer = &mut ctx.accounts.signer;

    // If this snapshot has no entries, then close immediately
    if snapshot.total_workers == 0 {
        let snapshot_lamports = snapshot.to_account_info().lamports();
        **snapshot.to_account_info().lamports.borrow_mut() = 0;
        **signer.to_account_info().lamports.borrow_mut() = signer
            .to_account_info()
            .lamports()
            .checked_add(snapshot_lamports)
            .unwrap();
    } else {
        // Otherwise, set the status to closing
        snapshot.status = SnapshotStatus::Closing;
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
