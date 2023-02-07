pub use clockwork_automation_program::errors;
pub use clockwork_automation_program::program::AutomationProgram;
pub use clockwork_automation_program::ID;

pub mod state {
    pub use clockwork_automation_program::state::{
        AccountMetaData, Automation, AutomationAccount, AutomationResponse, AutomationSettings,
        ClockData, ExecContext, InstructionData, Trigger, TriggerContext,
    };
}

pub mod utils {
    pub use clockwork_automation_program::state::anchor_sighash;
    pub use clockwork_automation_program::state::PAYER_PUBKEY;
}

pub mod cpi {
    use anchor_lang::prelude::{CpiContext, Result};

    pub use clockwork_automation_program::cpi::accounts::{
        AutomationCreate, AutomationDelete, AutomationPause, AutomationReset, AutomationResume,
        AutomationUpdate, AutomationWithdraw,
    };

    pub fn automation_create<'info>(
        ctx: CpiContext<'_, '_, '_, 'info, AutomationCreate<'info>>,
        amount: u64,
        id: Vec<u8>,
        instructions: Vec<crate::state::InstructionData>,
        trigger: crate::state::Trigger,
    ) -> Result<()> {
        clockwork_automation_program::cpi::automation_create(ctx, amount, id, instructions, trigger)
    }

    pub fn automation_delete<'info>(
        ctx: CpiContext<'_, '_, '_, 'info, AutomationDelete<'info>>,
    ) -> Result<()> {
        clockwork_automation_program::cpi::automation_delete(ctx)
    }

    pub fn automation_pause<'info>(
        ctx: CpiContext<'_, '_, '_, 'info, AutomationPause<'info>>,
    ) -> Result<()> {
        clockwork_automation_program::cpi::automation_pause(ctx)
    }

    pub fn automation_resume<'info>(
        ctx: CpiContext<'_, '_, '_, 'info, AutomationResume<'info>>,
    ) -> Result<()> {
        clockwork_automation_program::cpi::automation_resume(ctx)
    }

    pub fn automation_reset<'info>(
        ctx: CpiContext<'_, '_, '_, 'info, AutomationReset<'info>>,
    ) -> Result<()> {
        clockwork_automation_program::cpi::automation_reset(ctx)
    }

    pub fn automation_update<'info>(
        ctx: CpiContext<'_, '_, '_, 'info, AutomationUpdate<'info>>,
        settings: crate::state::AutomationSettings,
    ) -> Result<()> {
        clockwork_automation_program::cpi::automation_update(ctx, settings)
    }

    pub fn automation_withdraw<'info>(
        ctx: CpiContext<'_, '_, '_, 'info, AutomationWithdraw<'info>>,
        amount: u64,
    ) -> Result<()> {
        clockwork_automation_program::cpi::automation_withdraw(ctx, amount)
    }
}
