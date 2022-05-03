use anchor_lang::prelude::*;

#[event]
pub struct TaskExecuted {
    pub bot: Pubkey,
    pub task: Pubkey,
}
