use anchor_lang::prelude::*;

#[event]
pub struct TaskExecuted {
    pub delegate: Pubkey,
    pub task: Pubkey,
    pub ts: i64,
}
