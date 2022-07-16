use {
    crate::state::{Config, Fee, Queue},
    anchor_lang::{
        prelude::*,
        system_program::{transfer, Transfer},
    },
    cronos_pool::state::Pool,
};

pub fn is_spam<'info>(
    clock: &Sysvar<Clock>,
    config: &Account<'info, Config>,
    fee: &mut Account<'info, Fee>,
    pool: &Account<'info, Pool>,
    queue: &mut Account<'info, Queue>,
    system_program: &Program<'info, System>,
    worker: &mut Signer<'info>,
) -> Result<bool> {
    let is_authorized = pool.clone().into_inner().workers.contains(&worker.key());

    let is_holdout_period = clock.unix_timestamp
        < queue
            .exec_at
            .unwrap()
            .checked_add(config.worker_holdout_period)
            .unwrap();

    // Penalize the worker for spamming during the holdout period
    if !is_authorized && is_holdout_period {
        transfer(
            CpiContext::new(
                system_program.to_account_info(),
                Transfer {
                    from: worker.to_account_info(),
                    to: fee.to_account_info(),
                },
            ),
            config.worker_spam_penalty,
        )?;
        return Ok(true);
    }

    Ok(false)
}
