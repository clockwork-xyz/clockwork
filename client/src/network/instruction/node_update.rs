use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    },
    InstructionData,
};
use clockwork_network::state::NodeSettings;

pub fn node_update(authority: Pubkey, node: Pubkey, settings: NodeSettings) -> Instruction {
    Instruction {
        program_id: clockwork_network::ID,
        accounts: vec![
            AccountMeta::new(authority, true),
            AccountMeta::new(node, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: clockwork_network::instruction::NodeUpdate { settings }.data(),
    }
}
