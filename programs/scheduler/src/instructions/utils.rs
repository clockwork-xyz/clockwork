use {
    crate::state::{Config, Queue},
    anchor_lang::prelude::*,
    clockwork_pool::state::Pool,
};

pub fn is_spam<'info>(
    config: &Account<'info, Config>,
    pool: &Account<'info, Pool>,
    queue: &mut Account<'info, Queue>,
    worker: &mut Signer<'info>,
) -> Result<bool> {
    let ts = Clock::get().unwrap().unix_timestamp;
    let is_authorized = pool.clone().into_inner().workers.contains(&worker.key());
    let is_grace_period = ts
        < queue
            .process_at
            .unwrap()
            .checked_add(config.grace_period)
            .unwrap();

    Ok(is_grace_period && !is_authorized)

    // Penalize the worker for spamming during the holdout period
    // if !is_authorized && is_grace_period {
    //     fee.pay_to_admin(config.spam_penalty, queue)?;
    //     return Ok(true);
    // }

    // Ok(false)
}
