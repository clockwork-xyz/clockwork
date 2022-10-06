use {
    super::Queue,
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::convert::TryFrom,
};

pub const SEED_FEE: &[u8] = b"fee";

// TODO Write functions to distribute the claimable balance out to Delegations

/// Escrows the lamport balance owed to a particular worker.
#[account]
#[derive(Debug)]
pub struct Fee {
    /// The number of lamports that have been collected by the worker for working in this pool.
    pub collected: u64,
    /// The number of lamports that are withheld as penalty because the worker submitted spam.
    pub penalty: u64,
    /// The pool that the worker participated in in.
    pub pool: u64,
    /// The worker who did the work.
    pub worker: Pubkey,
}

impl Fee {
    /// Derive the pubkey of a fee account.
    pub fn pubkey(pool: Pubkey, worker: Pubkey) -> Pubkey {
        Pubkey::find_program_address(&[SEED_FEE, pool.as_ref(), worker.as_ref()], &crate::ID).0
    }
}

impl TryFrom<Vec<u8>> for Fee {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Fee::try_deserialize(&mut data.as_slice())
    }
}

/// Trait for reading and writing to a fee account.
pub trait FeeAccount {
    /// Get the pubkey of the fee account.
    fn pubkey(&self) -> Pubkey;

    /// Initialize the account to hold fee object.
    fn init(&mut self, worker: Pubkey) -> Result<()>;

    /// Claim the balance.
    // fn claim_balance(&mut self, amount: u64, pay_to: &mut SystemAccount) -> Result<()>;

    /// Claim the penalty.
    // fn claim_penalty(&mut self, amount: u64, pay_to: &mut SystemAccount) -> Result<()>;

    /// Debits lamports from the queue and increments the claimable balance.
    fn escrow_balance(&mut self, amount: u64, queue: &mut Account<Queue>) -> Result<()>;

    /// Debits lamports from the queue and increments the penalty.
    fn escrow_penalty(&mut self, amount: u64, queue: &mut Account<Queue>) -> Result<()>;
}

impl FeeAccount for Account<'_, Fee> {
    fn pubkey(&self) -> Pubkey {
        Fee::pubkey(self.worker)
    }

    fn init(&mut self, worker: Pubkey) -> Result<()> {
        self.worker = worker;
        self.balance = 0;
        self.penalty = 0;
        Ok(())
    }

    // fn claim_balance(&mut self, amount: u64, pay_to: &mut SystemAccount) -> Result<()> {
    //     // Withdraw from the worker amount
    //     self.balance = self.balance.checked_sub(amount).unwrap();

    //     // Transfer lamports to the pay_to acccount
    //     **self.to_account_info().try_borrow_mut_lamports()? = self
    //         .to_account_info()
    //         .lamports()
    //         .checked_sub(amount)
    //         .unwrap();
    //     **pay_to.to_account_info().try_borrow_mut_lamports()? = pay_to
    //         .to_account_info()
    //         .lamports()
    //         .checked_add(amount)
    //         .unwrap();

    //     Ok(())
    // }

    // fn claim_penalty(&mut self, amount: u64, pay_to: &mut SystemAccount) -> Result<()> {
    //     // Withdraw from the admin balance
    //     self.penalty = self.penalty.checked_sub(amount).unwrap();

    //     // Transfer lamports to the pay_to acccount
    //     **self.to_account_info().try_borrow_mut_lamports()? = self
    //         .to_account_info()
    //         .lamports()
    //         .checked_sub(amount)
    //         .unwrap();
    //     **pay_to.to_account_info().try_borrow_mut_lamports()? = pay_to
    //         .to_account_info()
    //         .lamports()
    //         .checked_add(amount)
    //         .unwrap();

    //     Ok(())
    // }

    fn escrow_penalty(&mut self, amount: u64, queue: &mut Account<Queue>) -> Result<()> {
        // Transfer balance from queue to fee account
        self.penalty = self.penalty.checked_add(amount).unwrap();

        // Transfer lamports
        **queue.to_account_info().try_borrow_mut_lamports()? = queue
            .to_account_info()
            .lamports()
            .checked_sub(amount)
            .unwrap();
        **self.to_account_info().try_borrow_mut_lamports()? = self
            .to_account_info()
            .lamports()
            .checked_add(amount)
            .unwrap();

        Ok(())
    }

    fn escrow_balance(&mut self, amount: u64, queue: &mut Account<Queue>) -> Result<()> {
        // Transfer balance from queue to fee account
        self.balance = self.balance.checked_add(amount).unwrap();

        // Transfer lamports
        **queue.to_account_info().try_borrow_mut_lamports()? = queue
            .to_account_info()
            .lamports()
            .checked_sub(amount)
            .unwrap();
        **self.to_account_info().try_borrow_mut_lamports()? = self
            .to_account_info()
            .lamports()
            .checked_add(amount)
            .unwrap();

        Ok(())
    }
}
