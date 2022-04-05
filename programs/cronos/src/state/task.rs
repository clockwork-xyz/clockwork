use {
    super::{Config, Daemon, DaemonAccount, Fee},
    crate::{errors::CronosError, pda::PDA},
    anchor_lang::{prelude::borsh::BorshSchema, prelude::*, AnchorDeserialize},
    chrono::{DateTime, NaiveDateTime, Utc},
    cronos_cron::Schedule,
    solana_program::instruction::Instruction,
    std::{collections::HashSet, convert::TryFrom, str::FromStr},
};

pub const SEED_TASK: &[u8] = b"task";

/**
 * Task
 */

#[account]
#[derive(Debug)]
pub struct Task {
    pub bump: u8,
    pub daemon: Pubkey,
    pub delegates: HashSet<Pubkey>,
    pub exec_at: Option<i64>,
    pub id: u128,
    pub ixs: Vec<InstructionData>,
    pub schedule: String,
}

impl Task {
    pub fn pda(daemon: Pubkey, id: u128) -> PDA {
        Pubkey::find_program_address(
            &[SEED_TASK, daemon.as_ref(), id.to_be_bytes().as_ref()],
            &crate::ID,
        )
    }
}

impl TryFrom<Vec<u8>> for Task {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Task::try_deserialize(&mut data.as_slice())
    }
}

/**
 * TaskAccount
 */

pub trait TaskAccount {
    fn open(
        &mut self,
        bump: u8,
        clock: &Sysvar<Clock>,
        daemon: &mut Account<Daemon>,
        ixs: Vec<InstructionData>,
        schedule: String,
    ) -> Result<()>;

    fn close(&mut self, to: &mut Signer) -> Result<()>;

    fn exec(
        &mut self,
        account_infos: &[AccountInfo],
        bot: &mut Signer,
        config: &Account<Config>,
        daemon: &mut Account<Daemon>,
        fee: &mut Account<Fee>,
    ) -> Result<()>;

    fn next_exec_at(&self, ts: i64) -> Option<i64>;
}

impl TaskAccount for Account<'_, Task> {
    fn open(
        &mut self,
        bump: u8,
        clock: &Sysvar<Clock>,
        daemon: &mut Account<Daemon>,
        ixs: Vec<InstructionData>,
        schedule: String,
    ) -> Result<()> {
        // Reject the instruction if it has signers other than the daemon.
        // TODO Support multi-sig ixs
        for ix in ixs.iter() {
            for acc in ix.accounts.iter() {
                require!(
                    !acc.is_signer || acc.pubkey == daemon.key(),
                    CronosError::InvalidSignatory
                );
            }
        }

        // Initialize task account.
        self.bump = bump;
        self.daemon = daemon.key();
        self.id = daemon.task_count;
        self.ixs = ixs;
        self.schedule = schedule;

        // Move forward, one step in time
        self.exec_at = self.next_exec_at(clock.unix_timestamp);

        // Increment daemon task counter
        daemon.task_count = daemon.task_count.checked_add(1).unwrap();

        Ok(())
    }

    fn close(&mut self, to: &mut Signer) -> Result<()> {
        let lamports = self.to_account_info().lamports();
        **self.to_account_info().try_borrow_mut_lamports()? = self
            .to_account_info()
            .lamports()
            .checked_sub(lamports)
            .unwrap();
        **to.to_account_info().try_borrow_mut_lamports()? = to
            .to_account_info()
            .lamports()
            .checked_add(lamports)
            .unwrap();

        Ok(())
    }

    fn exec(
        &mut self,
        account_infos: &[AccountInfo],
        bot: &mut Signer,
        config: &Account<Config>,
        daemon: &mut Account<Daemon>,
        fee: &mut Account<Fee>,
    ) -> Result<()> {
        // Sign all of the task instructions
        for ix in &self.ixs {
            daemon.sign(&Instruction::from(ix), account_infos)?;
        }

        // Update the exec_at timestamp
        match self.exec_at {
            Some(exec_at) => self.exec_at = self.next_exec_at(exec_at),
            None => {}
        }

        // Pay automation fees.
        **daemon.to_account_info().try_borrow_mut_lamports()? = daemon
            .to_account_info()
            .lamports()
            .checked_sub(config.program_fee)
            .unwrap();
        **bot.to_account_info().try_borrow_mut_lamports()? = bot
            .to_account_info()
            .lamports()
            .checked_add(config.program_fee)
            .unwrap();

        // Pay program fees.
        **daemon.to_account_info().try_borrow_mut_lamports()? = daemon
            .to_account_info()
            .lamports()
            .checked_sub(config.program_fee)
            .unwrap();
        **fee.to_account_info().try_borrow_mut_lamports()? = fee
            .to_account_info()
            .lamports()
            .checked_add(config.program_fee)
            .unwrap();

        // Increment collectable fee balance.
        fee.balance = fee.balance.checked_add(config.program_fee).unwrap();

        Ok(())
    }

    fn next_exec_at(&self, ts: i64) -> Option<i64> {
        None
        // match Schedule::from_str(&self.schedule)
        //     .unwrap()
        //     .after(&DateTime::<Utc>::from_utc(
        //         NaiveDateTime::from_timestamp(ts, 0),
        //         Utc,
        //     ))
        //     .take(1)
        //     .next()
        // {
        //     Some(datetime) => Some(datetime.timestamp()),
        //     None => None,
        // }
    }
}

/**
 * InstructionData
 */

#[derive(AnchorDeserialize, AnchorSerialize, BorshSchema, Clone, Debug, PartialEq)]
pub struct InstructionData {
    /// Pubkey of the instruction processor that executes this instruction
    pub program_id: Pubkey,
    /// Metadata for what accounts should be passed to the instruction processor
    pub accounts: Vec<AccountMetaData>,
    /// Opaque data passed to the instruction processor
    pub data: Vec<u8>,
}

impl From<Instruction> for InstructionData {
    fn from(instruction: Instruction) -> Self {
        InstructionData {
            program_id: instruction.program_id,
            accounts: instruction
                .accounts
                .iter()
                .map(|a| AccountMetaData {
                    pubkey: a.pubkey,
                    is_signer: a.is_signer,
                    is_writable: a.is_writable,
                })
                .collect(),
            data: instruction.data,
        }
    }
}

impl From<&InstructionData> for Instruction {
    fn from(instruction: &InstructionData) -> Self {
        Instruction {
            program_id: instruction.program_id,
            accounts: instruction
                .accounts
                .iter()
                .map(|a| AccountMeta {
                    pubkey: a.pubkey,
                    is_signer: a.is_signer,
                    is_writable: a.is_writable,
                })
                .collect(),
            data: instruction.data.clone(),
        }
    }
}

impl TryFrom<Vec<u8>> for InstructionData {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Ok(
            borsh::try_from_slice_with_schema::<InstructionData>(data.as_slice())
                .map_err(|_err| ErrorCode::AccountDidNotDeserialize)?,
        )
    }
}

/**
 * AccountMetaData
 */

#[derive(AnchorDeserialize, AnchorSerialize, BorshSchema, Clone, Debug, PartialEq)]
pub struct AccountMetaData {
    /// An account's public key
    pub pubkey: Pubkey,
    /// True if an Instruction requires a Transaction signature matching `pubkey`.
    pub is_signer: bool,
    /// True if the `pubkey` can be loaded as a read-write account.
    pub is_writable: bool,
}
