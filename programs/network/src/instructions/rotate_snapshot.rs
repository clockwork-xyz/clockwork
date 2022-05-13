use anchor_lang::{solana_program::instruction::Instruction, system_program};
use cronos_scheduler::{state::{Action, Task, Queue}, response::CronosResponse};

use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::sysvar},
};

#[derive(Accounts)]
pub struct RotateSnapshot<'info> {
    #[account(mut, has_one = task)]
    pub action: Account<'info, Action>,

    #[account(
        seeds = [SEED_AUTHORITY], 
        bump,
        has_one = queue
    )]
    pub authority: Account<'info, Authority>,

    #[account(address = sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,

    #[account(seeds = [SEED_CONFIG], bump)]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [
            SEED_SNAPSHOT,
            registry.snapshot_count.checked_sub(1).unwrap().to_be_bytes().as_ref()
        ],
        bump,
    )]
    pub current_snapshot: Account<'info, Snapshot>,

    #[account(
        mut,
        seeds = [
            SEED_SNAPSHOT,
            registry.snapshot_count.to_be_bytes().as_ref()
        ],
        bump,
    )]
    pub next_snapshot: Account<'info, Snapshot>,

    #[account(signer, constraint = queue.owner == authority.key())]
    pub queue: Account<'info, Queue>,

    #[account(mut, seeds = [SEED_REGISTRY], bump)]
    pub registry: Account<'info, Registry>,

    #[account(has_one = queue)]
    pub task: Account<'info, Task>,
}


pub fn handler(ctx: Context<RotateSnapshot>) -> Result<CronosResponse> {
    msg!("Rotating snapshot!");

    // Get accounts
    let action = &mut ctx.accounts.action;
    let authority = &ctx.accounts.authority;
    let clock = &ctx.accounts.clock;
    let config = &ctx.accounts.config;
    let current_snapshot = &mut ctx.accounts.current_snapshot;
    let next_snapshot = &mut ctx.accounts.next_snapshot;
    let queue = &ctx.accounts.queue;
    let registry = &mut ctx.accounts.registry;
    let task = &ctx.accounts.task;

    // Rotate the snapshot
    let res = registry.rotate_snapshot(clock, Some(current_snapshot), next_snapshot);
    if res.is_err() {
        // Don't return the error from here
        msg!("Snapshot rotation failed: {:?}", res.err())
    }

    // Update the action for the next task invocation
    let next_next_snapshot_pubkey = Snapshot::pda(next_snapshot.id.checked_add(1).unwrap()).0;
    let start_snapshot_ix = Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta {
                pubkey: config.key(),
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: cronos_scheduler::delegate::ID,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: queue.key(),
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: registry.key(),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: next_next_snapshot_pubkey,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: system_program::ID,
                is_signer: false,
                is_writable: false,
            },
        ],
        data: sighash("global", "start_snapshot").into(),
    };
    let rotate_snapshot_ix = Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta {
                pubkey: action.key(),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: authority.key(),
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: sysvar::clock::ID,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: config.key(),
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: next_snapshot.key(),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: next_next_snapshot_pubkey,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: queue.key(),
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: registry.key(),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: task.key(),
                is_signer: false,
                is_writable: false,
            },
        ],
        data: sighash("global", "rotate_snapshot").into(),
    };


    Ok(CronosResponse {
        update_action_ixs: vec![start_snapshot_ix.into(), rotate_snapshot_ix.into()]
    })
}

fn sighash(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{}:{}", namespace, name);
    let mut sighash = [0u8; 8];
    sighash.copy_from_slice(
        &anchor_lang::solana_program::hash::hash(preimage.as_bytes()).to_bytes()[..8],
    );
    sighash
}
