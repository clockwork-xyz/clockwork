use {
    crate::objects::*,
    anchor_lang::{prelude::*, solana_program::instruction::Instruction},
    // clockwork_queue_program::objects::{CrankResponse, Queue, QueueAccount},
};

#[derive(Accounts)]
pub struct EntryClose<'info> {
    #[account(address = Authority::pubkey())]
    pub authority: Account<'info, Authority>,

    #[account(
        mut,
        seeds = [
            SEED_SNAPSHOT_ENTRY,
            entry.snapshot.as_ref(),
            entry.id.to_be_bytes().as_ref(),
        ],
        bump,
        has_one = snapshot,
    )]
    pub entry: Account<'info, SnapshotEntry>,

    #[account(
        mut,
        seeds = [
            SEED_SNAPSHOT,
            snapshot.id.to_be_bytes().as_ref(),
        ],
        bump,
        constraint = snapshot.status == SnapshotStatus::Closing,
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account(mut)]
    pub signer: Signer<'info>,
}

pub fn handler(ctx: Context<EntryClose>) -> Result<()> {
    // Get accounts
    let authority = &ctx.accounts.authority;
    let entry = &mut ctx.accounts.entry;
    let signer = &mut ctx.accounts.signer;
    let snapshot = &mut ctx.accounts.snapshot;

    // If snapshot is not closing, then noop and try again on next invocation.
    if snapshot.status != SnapshotStatus::Closing {
        return Ok(());
    }

    // Close the entry account.
    let entry_id = entry.id.clone();
    let entry_lamports = entry.to_account_info().lamports();
    **entry.to_account_info().lamports.borrow_mut() = 0;
    **signer.to_account_info().lamports.borrow_mut() = signer
        .to_account_info()
        .lamports()
        .checked_add(entry_lamports)
        .unwrap();

    // If this is the last entry of the snapshot, then also close the snapshot account.
    let snapshot_pubkey = snapshot.key().clone();
    let snapshot_node_count = snapshot.node_count.clone();
    if entry_id == snapshot_node_count.checked_sub(1).unwrap() {
        let snapshot_lamports = snapshot.to_account_info().lamports();
        **snapshot.to_account_info().lamports.borrow_mut() = 0;
        **signer.to_account_info().lamports.borrow_mut() = signer
            .to_account_info()
            .lamports()
            .checked_add(snapshot_lamports)
            .unwrap();
    }

    Ok(())

    // Use dynamic accounts to run with the next snapshot on the next invocation
    // let next_instruction = if entry_id < snapshot.node_count.checked_sub(1).unwrap() {
    //     let next_entry_pubkey =
    //         SnapshotEntry::pubkey(snapshot_pubkey, entry.id.checked_add(1).unwrap());
    //     Some(
    //         Instruction {
    //             program_id: crate::ID,
    //             accounts: vec![
    //                 AccountMeta::new_readonly(authority.key(), false),
    //                 AccountMeta::new(next_entry_pubkey, false),
    //                 AccountMeta::new(snapshot.key(), false),
    //                 AccountMeta::new(snapshot_queue.key(), false),
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
