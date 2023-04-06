use {solana_program::pubkey::Pubkey, std::fmt::Debug};

#[derive(Clone, Debug)]
#[derive(Default)]
pub struct PoolPosition {
    pub current_position: Option<u64>,
    pub workers: Vec<Pubkey>,
}


