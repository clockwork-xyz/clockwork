use crate::state::FeeAccount;

use {
    crate::state::{Config, Fee, Queue},
    anchor_lang::prelude::*,
    cronos_pool::state::Pool,
};

pub fn is_spam<'info>(
    clock: &Sysvar<Clock>,
    config: &Account<'info, Config>,
    fee: &mut Account<'info, Fee>,
    pool: &Account<'info, Pool>,
    queue: &mut Account<'info, Queue>,
    worker: &mut Signer<'info>,
) -> Result<bool> {
    let is_authorized = pool.clone().into_inner().workers.contains(&worker.key());
    let is_grace_period = clock.unix_timestamp
        < queue
            .exec_at
            .unwrap()
            .checked_add(config.grace_period)
            .unwrap();

    // Penalize the worker for spamming during the holdout period
    if !is_authorized && is_grace_period {
        fee.pay_to_admin(config.spam_penalty, queue)?;
        return Ok(true);
    }

    Ok(false)
}
