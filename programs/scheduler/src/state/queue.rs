use {
    super::InstructionData,
    crate::{errors::CronosError, responses::ExecResponse},
    anchor_lang::{
        prelude::*,
        solana_program::{
            instruction::Instruction,
            program::{get_return_data, invoke_signed},
        },
        AnchorDeserialize,
    },
    chrono::{DateTime, NaiveDateTime, Utc},
    cronos_cron::Schedule,
    std::{convert::TryFrom, str::FromStr},
};

pub const SEED_QUEUE: &[u8] = b"queue";

// #[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
// pub enum Trigger {
//     Cron {
//         schedule: String,
//     },
//     Slot {
//         repeat: Option<u64>,
//         slot: Option<u64>,
//     },
// }

/**
 * Queue
 */

#[account]
#[derive(Debug)]
pub struct Queue {
    pub authority: Pubkey,
    pub balance: u64,
    pub id: u128,
    pub process_at: Option<i64>,
    pub schedule: String,
    pub status: QueueStatus,
    pub task_count: u128,
    // pub trigger: Trigger,
}

impl Queue {
    pub fn pubkey(authority: Pubkey, id: u128) -> Pubkey {
        Pubkey::find_program_address(
            &[SEED_QUEUE, authority.as_ref(), id.to_be_bytes().as_ref()],
            &crate::ID,
        )
        .0
    }
}

impl TryFrom<Vec<u8>> for Queue {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Queue::try_deserialize(&mut data.as_slice())
    }
}

/**
 * QueueAccount
 */

pub trait QueueAccount {
    fn process(&mut self) -> Result<()>;

    fn new(
        &mut self,
        authority: Pubkey,
        clock: &Sysvar<Clock>,
        id: u128,
        schedule: String,
    ) -> Result<()>;

    fn next_process_at(&self, ts: i64) -> Option<i64>;

    fn roll_forward(&mut self) -> Result<()>;

    fn sign(
        &self,
        account_infos: &[AccountInfo],
        bump: u8,
        ix: &InstructionData,
    ) -> Result<Option<ExecResponse>>;
}

impl QueueAccount for Account<'_, Queue> {
    fn process(&mut self) -> Result<()> {
        // Validate the queue is pending
        require!(
            self.status == QueueStatus::Pending,
            CronosError::InvalidQueueStatus,
        );

        if self.task_count > 0 {
            // If there are actions, change the queue status to 'executing'
            self.status = QueueStatus::Processing { task_id: 0 };
        } else {
            // Otherwise, just roll forward the process_at timestamp
            self.roll_forward()?;
        }

        Ok(())
    }

    fn new(
        &mut self,
        authority: Pubkey,
        clock: &Sysvar<Clock>,
        id: u128,
        schedule: String,
    ) -> Result<()> {
        // Initialize queue account
        self.authority = authority.key();
        self.balance = 0;
        self.id = id;
        self.schedule = schedule;
        self.status = QueueStatus::Pending;
        self.task_count = 0;

        // Set process_at (schedule must be set first)
        self.process_at = self.next_process_at(clock.unix_timestamp);

        Ok(())
    }

    fn next_process_at(&self, ts: i64) -> Option<i64> {
        match Schedule::from_str(&self.schedule)
            .unwrap()
            .after(&DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(ts, 0),
                Utc,
            ))
            .take(1)
            .next()
        {
            Some(datetime) => Some(datetime.timestamp()),
            None => None,
        }
    }

    fn roll_forward(&mut self) -> Result<()> {
        self.status = QueueStatus::Pending;
        match self.process_at {
            Some(process_at) => self.process_at = self.next_process_at(process_at),
            None => (),
        };
        Ok(())
    }

    fn sign(
        &self,
        account_infos: &[AccountInfo],
        bump: u8,
        ix: &InstructionData,
    ) -> Result<Option<ExecResponse>> {
        invoke_signed(
            &Instruction::from(ix),
            account_infos,
            &[&[
                SEED_QUEUE,
                self.authority.as_ref(),
                self.id.to_be_bytes().as_ref(),
                &[bump],
            ]],
        )
        .map_err(|_err| CronosError::InnerIxFailed)?;

        match get_return_data() {
            None => Ok(None),
            Some((program_id, return_data)) => {
                if program_id != ix.program_id {
                    Err(CronosError::InvalidReturnData.into())
                } else {
                    Ok(Some(
                        ExecResponse::try_from_slice(return_data.as_slice())
                            .map_err(|_err| CronosError::InvalidExecResponse)?,
                    ))
                }
            }
        }
    }
}

/**
 * QueueStatus
 */

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum QueueStatus {
    Paused,
    Pending,
    Processing { task_id: u128 },
}
