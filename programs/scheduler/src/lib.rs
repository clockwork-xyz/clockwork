extern crate chrono;
extern crate cronos_cron;

pub mod errors;
pub mod events;
pub mod pda;
pub mod state;

mod instructions;

use anchor_lang::prelude::*;
use instructions::*;
use state::*;

declare_id!("954gq7DotHyMPGJs57abBFJtGUn5iTRmFLUJgmbXwdck");

#[program]
pub mod cronos_scheduler {
    use super::*;

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
        daemon_bump: u8,
        fee_bump: u8,
        registry_pubkey: Pubkey,
    ) -> Result<()> {
        initialize::handler(
            ctx,
            authority_bump,
            config_bump,
            daemon_bump,
            fee_bump,
            registry_pubkey,
        )
    }

    pub fn admin_task_new(
        ctx: Context<AdminTaskNew>,
        ixs: Vec<InstructionData>,
        schedule: String,
        bump: u8,
    ) -> Result<()> {
        admin_task_new::handler(ctx, ixs, schedule, bump)
    }

    pub fn admin_task_cancel(ctx: Context<AdminTaskCancel>) -> Result<()> {
        admin_task_cancel::handler(ctx)
    }

    pub fn daemon_new(ctx: Context<DaemonNew>, daemon_bump: u8, fee_bump: u8) -> Result<()> {
        daemon_new::handler(ctx, daemon_bump, fee_bump)
    }

    pub fn daemon_sign(ctx: Context<DaemonSign>, ix: InstructionData) -> Result<()> {
        daemon_sign::handler(ctx, ix)
    }

    pub fn task_cancel(ctx: Context<TaskCancel>) -> Result<()> {
        task_cancel::handler(ctx)
    }

    pub fn task_new(
        ctx: Context<TaskNew>,
        ixs: Vec<InstructionData>,
        schedule: String,
        bump: u8,
    ) -> Result<()> {
        task_new::handler(ctx, ixs, schedule, bump)
    }

    pub fn task_exec(ctx: Context<TaskExec>) -> Result<()> {
        task_exec::handler(ctx)
    }
}
