pub use clockwork_thread_program::errors;
pub use clockwork_thread_program::program::ThreadProgram;
pub use clockwork_thread_program::ID;

pub mod state {
    pub use clockwork_thread_program::state::{
        ClockData, ExecContext, SerializableAccount, SerializableInstruction, Thread,
        ThreadAccount, ThreadResponse, ThreadSettings, Trigger, TriggerContext,
    };
}

pub mod utils {
    pub use clockwork_thread_program::state::Equality;
    pub use clockwork_thread_program::state::PAYER_PUBKEY;
}

pub mod cpi {
    use anchor_lang::prelude::{CpiContext, Pubkey, Result};

    pub use clockwork_thread_program::cpi::accounts::{
        LookupTablesAdd, LookupTablesCreate, LookupTablesDelete,
        LookupTablesRemove, ThreadBigInstructionAdd, ThreadCreate, ThreadDelete, ThreadDummyIx,
        ThreadPause, ThreadReset, ThreadResume, ThreadUpdate, ThreadWithdraw,
    };

    pub fn thread_create<'info>(
        ctx: CpiContext<'_, '_, '_, 'info, ThreadCreate<'info>>,
        amount: u64,
        id: Vec<u8>,
        instructions: Vec<crate::state::SerializableInstruction>,
        trigger: crate::state::Trigger,
    ) -> Result<()> {
        clockwork_thread_program::cpi::thread_create(ctx, amount, id, instructions, trigger)
    }

    pub fn thread_delete<'info>(
        ctx: CpiContext<'_, '_, '_, 'info, ThreadDelete<'info>>,
    ) -> Result<()> {
        clockwork_thread_program::cpi::thread_delete(ctx)
    }

    pub fn thread_pause<'info>(
        ctx: CpiContext<'_, '_, '_, 'info, ThreadPause<'info>>,
    ) -> Result<()> {
        clockwork_thread_program::cpi::thread_pause(ctx)
    }

    pub fn thread_resume<'info>(
        ctx: CpiContext<'_, '_, '_, 'info, ThreadResume<'info>>,
    ) -> Result<()> {
        clockwork_thread_program::cpi::thread_resume(ctx)
    }

    pub fn thread_reset<'info>(
        ctx: CpiContext<'_, '_, '_, 'info, ThreadReset<'info>>,
    ) -> Result<()> {
        clockwork_thread_program::cpi::thread_reset(ctx)
    }

    pub fn thread_update<'info>(
        ctx: CpiContext<'_, '_, '_, 'info, ThreadUpdate<'info>>,
        settings: crate::state::ThreadSettings,
    ) -> Result<()> {
        clockwork_thread_program::cpi::thread_update(ctx, settings)
    }

    pub fn thread_withdraw<'info>(
        ctx: CpiContext<'_, '_, '_, 'info, ThreadWithdraw<'info>>,
        amount: u64,
    ) -> Result<()> {
        clockwork_thread_program::cpi::thread_withdraw(ctx, amount)
    }

    pub fn thread_lookup_tables_create<'info>(
        ctx: CpiContext<'_, '_, '_, 'info, LookupTablesCreate<'info>>,
        address_lookup_tables: Vec<Pubkey>,
    ) -> Result<()> {
        clockwork_thread_program::cpi::thread_lookup_tables_create(ctx, address_lookup_tables)
    }

    pub fn thread_lookup_tables_add<'info>(
        ctx: CpiContext<'_, '_, '_, 'info, LookupTablesAdd<'info>>,
        address_lookup_tables: Vec<Pubkey>,
    ) -> Result<()> {
        clockwork_thread_program::cpi::thread_lookup_tables_add(ctx, address_lookup_tables)
    }

    pub fn thread_lookup_tables_remove<'info>(
        ctx: CpiContext<'_, '_, '_, 'info, LookupTablesRemove<'info>>,
        index: u64,
    ) -> Result<()> {
        clockwork_thread_program::cpi::thread_lookup_tables_remove(ctx, index)
    }

    pub fn thread_lookup_tables_delete<'info>(
        ctx: CpiContext<'_, '_, '_, 'info, LookupTablesDelete<'info>>,
    ) -> Result<()> {
        clockwork_thread_program::cpi::thread_lookup_tables_delete(ctx)
    }

    pub fn thread_big_instruction_add<'info>(
        ctx: CpiContext<'_, '_, '_, 'info, ThreadBigInstructionAdd<'info>>,
        instruction_data: Vec<u8>
    ) -> Result<()> {
        clockwork_thread_program::cpi::thread_big_instruction_add(ctx, instruction_data)
    }

    pub fn thread_dummy_ix<'info>(
        ctx: CpiContext<'_, '_, '_, 'info, ThreadDummyIx<'info>>,
    ) -> Result<()> {
        clockwork_thread_program::cpi::thread_dummy_ix(ctx)
    }
}
