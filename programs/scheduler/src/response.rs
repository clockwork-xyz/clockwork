use anchor_lang::{prelude::borsh::BorshSchema, prelude::*, AnchorDeserialize, AnchorSerialize};

use crate::state::InstructionData;

#[derive(AnchorDeserialize, AnchorSerialize, BorshSchema, Clone, Debug)]
pub struct CronosResponse {
    pub update_action_ixs: Vec<InstructionData>,
}
