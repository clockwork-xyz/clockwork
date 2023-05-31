use std::mem::size_of;

use anchor_lang::{prelude::*, solana_program::system_program};

declare_id!("E7p5KFo8kKCDm6BUnWtnVFkQSYh6ZA6xaGAuvpv8NXTa");

#[program]
pub mod gasless_program {
    pub use super::*;

    pub fn payer_create<'info>(
        ctx: Context<PayerCreate>,
        authorized_spenders: Vec<Pubkey>,
        amount: u64,
    ) -> Result<()> {
        let authority = &ctx.accounts.authority;
        let payer = &mut ctx.accounts.payer;
        let system_program = &ctx.accounts.system_program;

        payer.bump = *ctx.bumps.get("payer").unwrap();
        payer.authority = authority.key();
        payer.authorized_spenders = authorized_spenders;

        anchor_lang::system_program::transfer(
            CpiContext::new(
                system_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: authority.to_account_info(),
                    to: payer.to_account_info(),
                },
            ),
            amount,
        )?;

        Ok(())
    }

    pub fn spend<'info>(_ctx: Context<Spend>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PayerCreate<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [
            SEED_PAYER,
            authority.key().as_ref(),
        ],
        bump,
        space = 8 + size_of::<Payer>(),
        payer = authority
    )]
    pub payer: Account<'info, Payer>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Spend<'info> {
    #[account()]
    pub spender: Signer<'info>,

    #[account(
        mut,
        seeds = [
            SEED_PAYER,
            payer.authority.key().as_ref(),
        ],
        bump = payer.bump,
        constraint = payer.authorized_spenders.contains(&spender.key()) @ GaslessError::UnauthorizedSpender,
    )]
    pub payer: Account<'info, Payer>,
}

pub const SEED_PAYER: &[u8] = b"payer";

#[account]
#[derive(Debug)]
pub struct Payer {
    pub authority: Pubkey,
    pub authorized_spenders: Vec<Pubkey>,
    pub bump: u8,
}

#[error_code]
pub enum GaslessError {
    #[msg("This signer is not authorized to spend from this PDA")]
    UnauthorizedSpender,
}
