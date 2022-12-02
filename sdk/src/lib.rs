pub use clockwork_thread_program::errors;

pub use clockwork_thread_program::ID;

pub mod state {
    pub use clockwork_thread_program::state::{
        AccountMetaData, ClockData, ExecContext, InstructionData, Thread, ThreadAccount,
        ThreadResponse, ThreadSettings, Trigger, TriggerContext,
    };
}

pub mod utils {
    pub use clockwork_thread_program::state::anchor_sighash;
    pub use clockwork_thread_program::state::PAYER_PUBKEY;
}

#[cfg(feature = "cpi")]
pub mod cpi {
    use anchor_lang::prelude::{CpiContext, Result};

    pub fn thread_create<'info>(
        ctx: CpiContext<
            '_,
            '_,
            '_,
            'info,
            clockwork_thread_program::cpi::accounts::ThreadCreate<'info>,
        >,
        id: String,
        kickoff_ix: clockwork_thread_program::state::InstructionData,
        trigger: clockwork_thread_program::state::Trigger,
    ) -> Result<()> {
        clockwork_thread_program::cpi::thread_create(ctx, id, kickoff_ix, trigger)
    }

    pub fn thread_delete<'info>(
        ctx: CpiContext<
            '_,
            '_,
            '_,
            'info,
            clockwork_thread_program::cpi::accounts::ThreadDelete<'info>,
        >,
    ) -> Result<()> {
        clockwork_thread_program::cpi::thread_delete(ctx)
    }

    pub fn thread_pause<'info>(
        ctx: CpiContext<
            '_,
            '_,
            '_,
            'info,
            clockwork_thread_program::cpi::accounts::ThreadPause<'info>,
        >,
    ) -> Result<()> {
        clockwork_thread_program::cpi::thread_pause(ctx)
    }

    pub fn thread_resume<'info>(
        ctx: CpiContext<
            '_,
            '_,
            '_,
            'info,
            clockwork_thread_program::cpi::accounts::ThreadResume<'info>,
        >,
    ) -> Result<()> {
        clockwork_thread_program::cpi::thread_resume(ctx)
    }

    pub fn thread_stop<'info>(
        ctx: CpiContext<
            '_,
            '_,
            '_,
            'info,
            clockwork_thread_program::cpi::accounts::ThreadStop<'info>,
        >,
    ) -> Result<()> {
        clockwork_thread_program::cpi::thread_stop(ctx)
    }

    pub fn thread_update<'info>(
        ctx: CpiContext<
            '_,
            '_,
            '_,
            'info,
            clockwork_thread_program::cpi::accounts::ThreadUpdate<'info>,
        >,
        settings: clockwork_thread_program::state::ThreadSettings,
    ) -> Result<()> {
        clockwork_thread_program::cpi::thread_update(ctx, settings)
    }

    pub fn thread_withdraw<'info>(
        ctx: CpiContext<
            '_,
            '_,
            '_,
            'info,
            clockwork_thread_program::cpi::accounts::ThreadWithdraw<'info>,
        >,
        amount: u64,
    ) -> Result<()> {
        clockwork_thread_program::cpi::thread_withdraw(ctx, amount)
    }
}
