use {
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::convert::TryFrom,
};

// TODO is fee account necessary?
// TODO can this just be an extra field on the manager account?

pub const SEED_FEE: &[u8] = b"fee";

/**
 * Fee
 */

#[account]
#[derive(Debug)]
pub struct Fee {
    pub balance: u64,
    pub queue: Pubkey,
}

impl TryFrom<Vec<u8>> for Fee {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Fee::try_deserialize(&mut data.as_slice())
    }
}

impl Fee {
    pub fn pubkey(queue: Pubkey) -> Pubkey {
        Pubkey::find_program_address(&[SEED_FEE, queue.as_ref()], &crate::ID).0
    }
}

/**
 * FeeAccount
 */

pub trait FeeAccount {
    fn new(&mut self, queue: Pubkey) -> Result<()>;

    fn collect(&mut self, to: &mut Signer) -> Result<()>;
}

impl FeeAccount for Account<'_, Fee> {
    fn new(&mut self, queue: Pubkey) -> Result<()> {
        self.balance = 0;
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
