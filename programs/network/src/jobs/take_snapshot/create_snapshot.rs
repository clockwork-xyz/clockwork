use std::mem::size_of;

use anchor_lang::{prelude::*, solana_program::system_program, InstructionData};
use anchor_spl::associated_token::get_associated_token_address;
use clockwork_utils::automation::{
    AutomationResponse, SerializableAccount, SerializableInstruction, PAYER_PUBKEY,
};

use crate::{instruction, state::*};

#[derive(Accounts)]
pub struct TakeSnapshotCreateSnapshot<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        address = Registry::pubkey(),
        constraint = registry.locked
    )]
    pub registry: Account<'info, Registry>,

    #[account(
        init,
        seeds = [
            SEED_SNAPSHOT,
            registry.current_epoch.checked_add(1).unwrap().to_be_bytes().as_ref(),
        ],
        bump,
        space = 8 + size_of::<Snapshot>(),
        payer = payer
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = config.epoch_automation)]
    pub automation: Signer<'info>,
}

pub fn handler(ctx: Context<TakeSnapshotCreateSnapshot>) -> Result<AutomationResponse> {
    // Get accounts
    let config = &ctx.accounts.config;
    let registry = &ctx.accounts.registry;
    let snapshot = &mut ctx.accounts.snapshot;
    let system_program = &ctx.accounts.system_program;
    let automation = &ctx.accounts.automation;

    // Start a new snapshot.
    snapshot.init(registry.current_epoch.checked_add(1).unwrap())?;

    Ok(AutomationResponse {
        dynamic_instruction: if registry.total_workers.gt(&0) {
            // The registry has workers. Create a snapshot frame for the zeroth worker.
            let snapshot_frame_pubkey = SnapshotFrame::pubkey(snapshot.key(), 0);
            let worker_pubkey = Worker::pubkey(0);
            Some(SerializableInstruction {
                program_id: crate::ID,
                accounts: vec![
                    SerializableAccount::readonly(config.key(), false),
                    SerializableAccount::mutable(PAYER_PUBKEY, true),
                    SerializableAccount::readonly(registry.key(), false),
                    SerializableAccount::mutable(snapshot.key(), false),
                    SerializableAccount::mutable(snapshot_frame_pubkey, false),
                    SerializableAccount::readonly(system_program.key(), false),
                    SerializableAccount::readonly(automation.key(), true),
                    SerializableAccount::readonly(worker_pubkey, false),
                    SerializableAccount::readonly(
                        get_associated_token_address(&worker_pubkey, &config.mint),
                        false,
                    ),
                ],
                data: instruction::TakeSnapshotCreateFrame {}.data(),
            })
        } else {
            None
        },
        trigger: None,
    })
}
