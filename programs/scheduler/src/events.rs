use anchor_lang::prelude::*;

#[event]
pub struct QueueExecuted {
    pub delegate: Pubkey,
    pub queue: Pubkey,
    pub ts: i64,
}
