pub mod clock;
pub mod cronos;
pub mod heartbeat;

pub use cronos_program::errors;
pub use cronos_program::pda;

// Export current solana-program types for downstream users who may also be
// building with a different solana-program version
pub use solana_program;

// The library uses this to verify the keys
solana_program::declare_id!(cronos_program::ID);
