use anchor_lang::{prelude::*, AnchorDeserialize, AnchorSerialize};

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug)]
pub struct ExecResponse {
    pub dynamic_accounts: Vec<Pubkey>,
}
