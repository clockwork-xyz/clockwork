use std::mem::size_of;

use anchor_lang::{prelude::*, solana_program::system_program};
use clockwork_network_program::state::{Fee, Pool, Worker, WorkerAccount};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

/// The ID of the pool workers must be a member of to collect fees.
const POOL_ID: u64 = 1;

#[program]
pub mod gasless {
    pub use super::*;

    pub fn payer_create<'info>(ctx: Context<PayerCreate>, amount: u64) -> Result<()> {
        let authority = &ctx.accounts.authority;
        let payer = &mut ctx.accounts.payer;
        let spender = &ctx.accounts.spender;
        let system_program = &ctx.accounts.system_program;

        // Initialize payer account.
        payer.bump = *ctx.bumps.get("payer").unwrap();
        payer.authority = authority.key();
        payer.spenders = vec![spender.key()];

        // Fund the payer account.
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

    pub fn add_spender<'info>(ctx: Context<AddSpender>) -> Result<()> {
        let authority = &ctx.accounts.authority;
        let payer = &mut ctx.accounts.payer;
        let spender = &ctx.accounts.spender;
        let system_program = &ctx.accounts.system_program;

        // Add authorized spender.
        payer.spenders.push(spender.key());

        // Realloc account size.
        let new_data_len = payer.to_account_info().data_len().checked_add(32).unwrap();
        payer.to_account_info().realloc(new_data_len, false)?;

        // Pay min rent requirement.
        let rent_requirement = Rent::get().unwrap().minimum_balance(new_data_len);
        if rent_requirement.gt(&payer.to_account_info().lamports()) {
            anchor_lang::system_program::transfer(
                CpiContext::new(
                    system_program.to_account_info(),
                    anchor_lang::system_program::Transfer {
                        from: authority.to_account_info(),
                        to: payer.to_account_info(),
                    },
                ),
                rent_requirement.saturating_sub(payer.to_account_info().lamports()),
            )?;
        }

        Ok(())
    }

    pub fn spend<'info>(_ctx: Context<Spend>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(amount: u64)]
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
        space = 8 + size_of::<Payer>() + 32,
        payer = authority
    )]
    pub payer: Account<'info, Payer>,

    #[account()]
    pub spender: SystemAccount<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddSpender<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account()]
    pub spender: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [
            SEED_PAYER,
            authority.key().as_ref(),
        ],
        bump = payer.bump,
    )]
    pub payer: Account<'info, Payer>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Spend<'info> {
    #[account(mut)]
    pub fee: Account<'info, Fee>,

    #[account()]
    pub spender: Signer<'info>,

    #[account(mut)]
    pub signatory: Signer<'info>,

    #[account(
        mut,
        seeds = [
            SEED_PAYER,
            payer.authority.key().as_ref(),
        ],
        bump = payer.bump,
        constraint = payer.spenders.contains(&spender.key()) @ GaslessError::UnauthorizedSpender,
    )]
    pub payer: Account<'info, Payer>,
}

pub const SEED_PAYER: &[u8] = b"payer";

#[account]
#[derive(Debug)]
pub struct Payer {
    pub authority: Pubkey,
    pub spenders: Vec<Pubkey>,
    pub bump: u8,
}

#[error_code]
pub enum GaslessError {
    #[msg("This signer is not authorized to spend from this PDA")]
    UnauthorizedSpender,
}
