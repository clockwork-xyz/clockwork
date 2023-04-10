use anchor_lang::solana_program::entrypoint::ProgramResult;

clockwork_anchor_gen::generate_cpi_interface!(idl_path = "idl.json");

anchor_lang::prelude::declare_id!("3XXuUFfweXBwFgFfYaejLvZE4cGZiHgKiGfMtdxNzYmv");

pub const SEED_THREAD: &[u8] = b"thread";

impl Thread {
    /// Derive the pubkey of a thread account.
    pub fn pubkey(authority: Pubkey, id: String) -> Pubkey {
        Pubkey::find_program_address(
            &[SEED_THREAD, authority.as_ref(), id.as_bytes()],
            &crate::ID,
        )
        .0
    }
}

impl PartialEq for Thread {
    fn eq(&self, other: &Self) -> bool {
        self.authority.eq(&other.authority) && self.id.eq(&other.id)
    }
}
