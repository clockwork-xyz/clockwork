// Utils
pub use clockwork_thread_program::objects::{
    anchor_sighash, AccountMetaData, ClockData, ExecResponse, InstructionData, PAYER_PUBKEY,
};

// For clients
#[cfg(feature = "client")]
pub mod client {
    pub use clockwork_client::{
        thread as thread_program, Client, ClientError, ClientResult, SplToken,
    };
}

// For programs that need to CPI into Clockwork.
#[cfg(feature = "thread")]
pub mod thread_program {
    pub use clockwork_thread_program::{cpi, errors, program::ThreadProgram, ID};
    pub mod accounts {
        pub use clockwork_thread_program::objects::{
            ExecContext, Thread, ThreadAccount, ThreadSettings, Trigger, TriggerContext,
        };
    }
}
