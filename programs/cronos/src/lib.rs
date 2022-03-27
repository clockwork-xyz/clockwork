extern crate chrono;
extern crate cronos_cron;

pub mod errors;
pub mod pda;
pub mod state;

mod instructions;

use anchor_lang::prelude::*;
use instructions::*;
use state::*;

declare_id!("CronpZj5NbHj2Nb6WwEtf6A9anty9JfEQ1RnGoshQBaW");

#[program]
pub mod cronos {
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

    pub fn admin_health_reset(ctx: Context<AdminHealthReset>) -> Result<()> {
        admin_health_reset::handler(ctx)
    }

    pub fn admin_open(
        ctx: Context<AdminOpen>,
        authority_bump: u8,
        config_bump: u8,
        daemon_bump: u8,
        fee_bump: u8,
        health_bump: u8,
    ) -> Result<()> {
        admin_open::handler(
            ctx,
            authority_bump,
            config_bump,
            daemon_bump,
            fee_bump,
            health_bump,
        )
    }

    pub fn admin_task_open(
        ctx: Context<AdminTaskOpen>,
        ixs: Vec<InstructionData>,
        schedule: String,
        bump: u8,
    ) -> Result<()> {
        admin_task_open::handler(ctx, ixs, schedule, bump)
    }

    pub fn admin_task_close(ctx: Context<AdminTaskClose>) -> Result<()> {
        admin_task_close::handler(ctx)
    }

    pub fn daemon_open(ctx: Context<DaemonOpen>, daemon_bump: u8, fee_bump: u8) -> Result<()> {
        daemon_open::handler(ctx, daemon_bump, fee_bump)
    }

    pub fn daemon_sign(ctx: Context<DaemonSign>, ix: InstructionData) -> Result<()> {
        daemon_sign::handler(ctx, ix)
    }

    pub fn daemon_close(ctx: Context<DaemonClose>) -> Result<()> {
        daemon_close::handler(ctx)
    }

    pub fn health_ping(ctx: Context<HealthPing>) -> Result<()> {
        health_ping::handler(ctx)
    }

    pub fn task_close(ctx: Context<TaskClose>) -> Result<()> {
        task_close::handler(ctx)
    }

    pub fn task_open(
        ctx: Context<TaskOpen>,
        ixs: Vec<InstructionData>,
        schedule: String,
        bump: u8,
    ) -> Result<()> {
        task_open::handler(ctx, ixs, schedule, bump)
    }

    pub fn task_exec(ctx: Context<TaskExec>) -> Result<()> {
        task_exec::handler(ctx)
    }
}
