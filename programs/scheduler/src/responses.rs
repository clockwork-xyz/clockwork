use {
    anchor_lang::{prelude::*, AnchorDeserialize, AnchorSerialize},
    std::default::Default,
};

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug)]
pub struct TaskResponse {
    pub dynamic_accounts: Option<Vec<Pubkey>>,
}

impl Default for TaskResponse {
    fn default() -> Self {
        return Self {
            dynamic_accounts: None,
        };
    }
}
