extern crate chrono;
extern crate cronos_cron;

pub mod errors;
pub mod events;
pub mod id;
pub mod pda;
pub mod state;

mod instructions;

pub use id::ID;

use anchor_lang::prelude::*;
use instructions::*;
use state::*;

#[program]
pub mod cronos_scheduler {
    use super::*;

    pub fn action_new(ctx: Context<ActionNew>, bump: u8, ixs: Vec<InstructionData>) -> Result<()> {
        action_new::handler(ctx, bump, ixs)
    }

    pub fn admin_config_update(
        ctx: Context<AdminConfigUpdate>,
        settings: ConfigSettings,
    ) -> Result<()> {
        admin_config_update::handler(ctx, settings)
    }

    pub fn admin_fee_collect(ctx: Context<AdminFeeCollect>) -> Result<()> {
        admin_fee_collect::handler(ctx)
    }

    pub fn initialize(
        ctx: Context<Initialize>,
        authority_bump: u8,
        config_bump: u8,
        fee_bump: u8,
        pool_pubkey: Pubkey,
        queue_bump: u8,
    ) -> Result<()> {
        initialize::handler(
            ctx,
            authority_bump,
            config_bump,
            fee_bump,
            pool_pubkey,
            queue_bump,
        )
    }

    pub fn admin_task_new(ctx: Context<AdminTaskNew>, schedule: String, bump: u8) -> Result<()> {
        admin_task_new::handler(ctx, schedule, bump)
    }

    pub fn admin_task_cancel(ctx: Context<AdminTaskCancel>) -> Result<()> {
        admin_task_cancel::handler(ctx)
    }

    pub fn queue_new(ctx: Context<QueueNew>) -> Result<()> {
        queue_new::handler(ctx)
    }

    pub fn queue_sign(ctx: Context<QueueSign>, ix: InstructionData) -> Result<()> {
        queue_sign::handler(ctx, ix)
    }

    pub fn task_cancel(ctx: Context<TaskCancel>) -> Result<()> {
        task_cancel::handler(ctx)
    }

    pub fn task_new(ctx: Context<TaskNew>, schedule: String) -> Result<()> {
        task_new::handler(ctx, schedule)
    }

    pub fn task_exec(ctx: Context<TaskExec>) -> Result<()> {
        task_exec::handler(ctx)
    }
}
