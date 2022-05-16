use {
    super::{AccountMetaData, Config, Fee, InstructionData, Task, Yogi, YogiAccount},
    crate::{errors::CronosError, pda::PDA},
    anchor_lang::{prelude::*, AnchorDeserialize},
    chrono::{DateTime, NaiveDateTime, Utc},
    cronos_cron::Schedule,
    std::{convert::TryFrom, str::FromStr},
};

pub const SEED_QUEUE: &[u8] = b"queue";

/**
 * Queue
 */

#[account]
#[derive(Debug)]
pub struct Queue {
    pub task_count: u128,
    pub exec_at: Option<i64>,
    pub id: u128,
    pub yogi: Pubkey,
    pub schedule: String,
    pub status: QueueStatus,
}

impl Queue {
    pub fn pda(yogi: Pubkey, id: u128) -> PDA {
        Pubkey::find_program_address(
            &[SEED_QUEUE, yogi.as_ref(), id.to_be_bytes().as_ref()],
            &crate::ID,
        )
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
    fn begin(&mut self) -> Result<()>;

    fn cancel(&mut self, to: &mut Signer) -> Result<()>;

    fn end(&mut self) -> Result<()>;

    fn exec(
        &mut self,
        account_infos: &Vec<AccountInfo>,
        task: &mut Account<Task>,
        delegate: &mut Signer,
        config: &Account<Config>,
        fee: &mut Account<Fee>,
        yogi: &Account<Yogi>,
    ) -> Result<()>;

    fn new(
        &mut self,
        clock: &Sysvar<Clock>,
        yogi: &mut Account<Yogi>,
        schedule: String,
    ) -> Result<()>;

    fn next_exec_at(&self, ts: i64) -> Option<i64>;

    fn roll_forward(&mut self) -> Result<()>;
}

impl QueueAccount for Account<'_, Queue> {
    fn begin(&mut self) -> Result<()> {
        // Validate the queue is pending
        require!(
            self.status == QueueStatus::Pending,
            CronosError::InvalidQueueStatus,
        );

        if self.task_count > 0 {
            // If there are actions, change the queue status to 'executing'
            self.status = QueueStatus::Executing { task_id: 0 };
        } else {
            // Otherwise, just roll forward the exec_at timestamp
            self.roll_forward()?;
        }

        Ok(())
    }

    fn cancel(&mut self, to: &mut Signer) -> Result<()> {
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

    fn end(&mut self) -> Result<()> {
        self.status = QueueStatus::Pending;
        self.roll_forward()
    }

    fn exec(
        &mut self,
        account_infos: &Vec<AccountInfo>,
        task: &mut Account<Task>,
        delegate: &mut Signer,
        config: &Account<Config>,
        fee: &mut Account<Fee>,
        yogi: &Account<Yogi>,
    ) -> Result<()> {
        // Validate the task id matches the queue's current execution state
        require!(
            task.id
                == match self.status {
                    QueueStatus::Executing { task_id } => task_id,
                    _ => return Err(CronosError::InvalidQueueStatus.into()),
                },
            CronosError::InvalidTask
        );

        // Validate the delegate data is empty
        require!(delegate.data_is_empty(), CronosError::DelegateDataNotEmpty);

        // Record the delegate's lamports before invoking inner ixs
        let delegate_lamports_pre = delegate.lamports();

        // Create an array of dynamic ixs to update the task for the next invocation
        let dyanmic_ixs: &mut Vec<InstructionData> = &mut vec![];

        // Process all of the task instructions
        for ix in &task.ixs {
            // If an inner ix account matches the Cronos delegate address (CronosDe1egate11111111111111111111111111111),
            //  then inject the delegate account in its place. Dapp developers can use the delegate as a payer to initialize
            //  new accouns in their queues. Delegates will be reimbursed for all SOL spent during the inner ixs.
            //
            // Because the delegate can be injected as the signer on inner ixs (written by presumed malicious parties),
            //  node operators should not secure any assets or staking positions with their delegate wallets other than
            //  an operational level of lamports needed to submit txns (~0.1 ⊚).
            //
            // TODO Update the network program to allow for split identity / delegate addresses so CRON stakes
            //  are not secured by delegate signatures.
            let accs: &mut Vec<AccountMetaData> = &mut vec![];
            ix.accounts.iter().for_each(|acc| {
                if acc.pubkey == crate::delegate::ID {
                    accs.push(AccountMetaData {
                        pubkey: delegate.key(),
                        is_signer: acc.is_signer,
                        is_writable: acc.is_writable,
                    });
                } else {
                    accs.push(acc.clone());
                }
            });

            // Execute the inner ix and process the response. Note that even though the yogi PDA is a signer
            //  on this ix, Solana will not allow downstream programs to mutate accounts owned by this program
            //  and explicitly forbids CPI reentrancy.
            let exec_response = yogi.process(
                &InstructionData {
                    program_id: ix.program_id,
                    accounts: accs.clone(),
                    data: ix.data.clone(),
                },
                account_infos,
            )?;

            match exec_response.dynamic_accounts {
                None => (), // Noop
                Some(dynamic_accounts) => {
                    require!(
                        dynamic_accounts.len() == ix.accounts.len(),
                        CronosError::InvalidDynamicAccounts
                    );
                    dyanmic_ixs.push(InstructionData {
                        program_id: ix.program_id,
                        accounts: dynamic_accounts
                            .iter()
                            .enumerate()
                            .map(|(i, pubkey)| {
                                let acc = ix.accounts.get(i).unwrap();
                                AccountMetaData {
                                    pubkey: *pubkey,
                                    is_signer: acc.is_signer,
                                    is_writable: acc.is_writable,
                                }
                            })
                            .collect::<Vec<AccountMetaData>>(),
                        data: ix.data.clone(),
                    });
                }
            }
        }

        // Verify that inner ixs have not initialized data at the delegate address
        require!(delegate.data_is_empty(), CronosError::DelegateDataNotEmpty);

        // Update the actions's ixs for the next invocation
        if !dyanmic_ixs.is_empty() {
            task.ixs = dyanmic_ixs.clone();
        }

        // Track how many lamports the delegate spent in the inner ixs
        let delegate_lamports_post = delegate.lamports();
        let delegate_reimbursement = delegate_lamports_pre
            .checked_sub(delegate_lamports_post)
            .unwrap();

        // Pay delegate fees
        let total_delegate_fee = config
            .delegate_fee
            .checked_add(delegate_reimbursement)
            .unwrap();
        **yogi.to_account_info().try_borrow_mut_lamports()? = yogi
            .to_account_info()
            .lamports()
            .checked_sub(total_delegate_fee)
            .unwrap();
        **delegate.to_account_info().try_borrow_mut_lamports()? = delegate
            .to_account_info()
            .lamports()
            .checked_add(total_delegate_fee)
            .unwrap();

        // Pay program fees
        **yogi.to_account_info().try_borrow_mut_lamports()? = yogi
            .to_account_info()
            .lamports()
            .checked_sub(config.program_fee)
            .unwrap();
        **fee.to_account_info().try_borrow_mut_lamports()? = fee
            .to_account_info()
            .lamports()
            .checked_add(config.program_fee)
            .unwrap();

        // Increment collectable fee balance
        fee.balance = fee.balance.checked_add(config.program_fee).unwrap();

        // Update the queue status
        let next_task_id = task.id.checked_add(1).unwrap();
        if next_task_id == self.task_count {
            self.end()?;
        } else {
            self.status = QueueStatus::Executing {
                task_id: next_task_id,
            };
        }

        Ok(())
    }

    fn new(
        &mut self,
        clock: &Sysvar<Clock>,
        yogi: &mut Account<Yogi>,
        schedule: String,
    ) -> Result<()> {
        // Initialize queue account
        self.task_count = 0;
        self.id = yogi.queue_count;
        self.yogi = yogi.key();
        self.schedule = schedule;
        self.status = QueueStatus::Pending;

        // Set exec_at (schedule must be set first)
        self.exec_at = self.next_exec_at(clock.unix_timestamp);

        // Increment yogi queue counter
        yogi.queue_count = yogi.queue_count.checked_add(1).unwrap();

        Ok(())
    }

    fn next_exec_at(&self, ts: i64) -> Option<i64> {
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
        match self.exec_at {
            Some(exec_at) => self.exec_at = self.next_exec_at(exec_at),
            None => (),
        };
        Ok(())
    }
}

/**
 * QueueStatus
 */

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum QueueStatus {
    Executing { task_id: u128 },
    Paused,
    Pending,
}
