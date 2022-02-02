mod instructions;
pub mod state;

use {anchor_lang::prelude::*, instructions::*};

declare_id!("4jnuXU4zsBavciDpTNmJ8dtwMBSkhKn2EBCiE9Rrv8uQ");

#[program]
pub mod indexer {
    use super::*;

    pub fn create_list(ctx: Context<CreateList>, bump: u8) -> ProgramResult {
        create_list::handler(ctx, bump)
    }

    pub fn delete_list(ctx: Context<DeleteList>) -> ProgramResult {
        delete_list::handler(ctx)
    }

    pub fn pop_element(ctx: Context<PopElement>) -> ProgramResult {
        pop_element::handler(ctx)
    }

    pub fn push_element(ctx: Context<PushElement>, value: Pubkey, bump: u8) -> ProgramResult {
        push_element::handler(ctx, value, bump)
    }
}
