use {crate::objects::*, anchor_lang::prelude::*};

#[derive(Accounts)]
pub struct EntryClose<'info> {
    #[account(
        mut,
        seeds = [
            SEED_SNAPSHOT,
            snapshot.epoch.as_ref(),
        ],
        bump,
        // constraint = snapshot.status == SnapshotStatus::Closing,
    )]
    pub snapshot: Account<'info, Snapshot>,

    // #[account(
    //     mut,
    //     seeds = [
    //         SEED_SNAPSHOT_ENTRY,
    //         snapshot_frame.snapshot.as_ref(),
    //         snapshot_frame.id.to_be_bytes().as_ref(),
    //     ],
    //     bump,
    //     has_one = snapshot,
    // )]
    // pub snapshot_frame: Account<'info, SnapshotEntry>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

pub fn handler(ctx: Context<EntryClose>) -> Result<()> {
    // Get accounts
    // let entry = &mut ctx.accounts.entry;
    let signer = &mut ctx.accounts.signer;
    let snapshot = &mut ctx.accounts.snapshot;

    // If snapshot is not closing, then noop and try again on next invocation.
    // if snapshot.status != SnapshotStatus::Closing {
    //     return Ok(());
    // }

    // Close the entry account.
    // let entry_id = entry.id.clone();
    // let entry_lamports = entry.to_account_info().lamports();
    // **entry.to_account_info().lamports.borrow_mut() = 0;
    // **signer.to_account_info().lamports.borrow_mut() = signer
    //     .to_account_info()
    //     .lamports()
    //     .checked_add(entry_lamports)
    //     .unwrap();

    // If this is the last entry of the snapshot, then also close the snapshot account.
    // let snapshot_total_workers = snapshot.total_workers.clone();
    // if entry_id == snapshot_total_workers.checked_sub(1).unwrap() {
    //     let snapshot_lamports = snapshot.to_account_info().lamports();
    //     **snapshot.to_account_info().lamports.borrow_mut() = 0;
    //     **signer.to_account_info().lamports.borrow_mut() = signer
    //         .to_account_info()
    //         .lamports()
    //         .checked_add(snapshot_lamports)
    //         .unwrap();
    // }

    Ok(())
}