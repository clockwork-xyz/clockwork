use anchor_lang::{prelude::*, AnchorDeserialize, AnchorSerialize};

pub const SEED_LOOKUP: &[u8] = b"lookup";

#[account]
#[derive(Debug)]
pub struct LookupTables {
    pub thread: Pubkey,
    pub authority: Pubkey,
    pub lookup_tables: Vec<Pubkey>,
    pub bump: u8,
}

impl LookupTables {
    /// Derive the pubkey of a lookup account.
    pub fn pubkey(authority: Pubkey, thread: Pubkey) -> Pubkey {
        Pubkey::find_program_address(
            &[SEED_LOOKUP, authority.as_ref(), thread.as_ref()],
            &crate::ID,
        )
        .0
    }
}

impl PartialEq for LookupTables {
    fn eq(&self, other: &Self) -> bool {
        self.authority.eq(&other.authority) && self.thread.eq(&other.thread)
    }
}

impl Eq for LookupTables {}

/// Trait for reading and writing to a lookuptables account.
pub trait LookupTablesAccount {
    /// Get the pubkey of the lookuptables account.
    fn pubkey(&self) -> Pubkey;

    /// Allocate more memory for the account.
    fn realloc(&mut self) -> Result<()>;
}

impl LookupTablesAccount for Account<'_, LookupTables> {
    fn pubkey(&self) -> Pubkey {
    LookupTables::pubkey(self.authority, self.thread)
    }

    fn realloc(&mut self) -> Result<()> {
        // Realloc memory for the lookuptables account
        let data_len = 8 + self.try_to_vec()?.len();
        self.to_account_info().realloc(data_len, false)?;
        Ok(())
    }
}