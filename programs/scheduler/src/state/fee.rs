use {
    crate::pda::PDA,
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::convert::TryFrom,
};

pub const SEED_FEE: &[u8] = b"fee";

/**
 * Fee
 */

#[account]
#[derive(Debug)]
pub struct Fee {
    pub balance: u64,
    pub bump: u8,
    pub queue: Pubkey,
}

impl TryFrom<Vec<u8>> for Fee {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Fee::try_deserialize(&mut data.as_slice())
    }
}

impl Fee {
    pub fn pda(queue: Pubkey) -> PDA {
        Pubkey::find_program_address(&[SEED_FEE, queue.as_ref()], &crate::ID)
    }
}

/**
 * FeeAccount
 */

pub trait FeeAccount {
    fn new(&mut self, bump: u8, queue: Pubkey) -> Result<()>;

    fn collect(&mut self, to: &mut Signer) -> Result<()>;
}

impl FeeAccount for Account<'_, Fee> {
    fn new(&mut self, bump: u8, queue: Pubkey) -> Result<()> {
        self.balance = 0;
        self.bump = bump;
        self.queue = queue;
        Ok(())
    }

    fn collect(&mut self, to: &mut Signer) -> Result<()> {
        **self.to_account_info().try_borrow_mut_lamports()? = self
            .to_account_info()
            .lamports()
            .checked_sub(self.balance)
            .unwrap();
        **to.to_account_info().try_borrow_mut_lamports()? = to
            .to_account_info()
            .lamports()
            .checked_add(self.balance)
            .unwrap();

        self.balance = 0;

        Ok(())
    }
}
