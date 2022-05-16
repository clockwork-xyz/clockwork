use super::InstructionData;

use {
    crate::{errors::CronosError, pda::PDA, responses::ExecResponse},
    anchor_lang::{
        prelude::*,
        solana_program::{
            instruction::Instruction,
            program::{get_return_data, invoke_signed},
        },
        AnchorDeserialize,
    },
    std::convert::TryFrom,
};

pub const SEED_YOGI: &[u8] = b"yogi";

/**
 * Yogi
 */

#[account]
#[derive(Debug)]
pub struct Yogi {
    pub owner: Pubkey,
    pub queue_count: u128,
    pub bump: u8,
}

impl Yogi {
    pub fn pda(owner: Pubkey) -> PDA {
        Pubkey::find_program_address(&[SEED_YOGI, owner.as_ref()], &crate::ID)
    }
}

impl TryFrom<Vec<u8>> for Yogi {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Yogi::try_deserialize(&mut data.as_slice())
    }
}

/**
 * YogiAccount
 */

pub trait YogiAccount {
    fn new(&mut self, bump: u8, owner: Pubkey) -> Result<()>;

    fn process(&self, ix: &InstructionData, account_infos: &[AccountInfo]) -> Result<ExecResponse>;
}

impl YogiAccount for Account<'_, Yogi> {
    fn new(&mut self, bump: u8, owner: Pubkey) -> Result<()> {
        self.bump = bump;
        self.owner = owner;
        self.queue_count = 0;
        Ok(())
    }

    fn process(&self, ix: &InstructionData, account_infos: &[AccountInfo]) -> Result<ExecResponse> {
        invoke_signed(
            &Instruction::from(ix),
            account_infos,
            &[&[SEED_YOGI, self.owner.as_ref(), &[self.bump]]],
        )
        .map_err(|_err| CronosError::InnerIxFailed)?;

        let exec_response = get_return_data()
            .ok_or(CronosError::InvalidExecResponse)
            .and_then(|(program_id, return_data)| {
                (program_id == ix.program_id)
                    .then(|| return_data)
                    .ok_or(CronosError::InvalidExecResponse)
            })
            .map(|return_data| {
                ExecResponse::try_from_slice(return_data.as_slice())
                    .map_err(|_err| CronosError::InvalidExecResponse)
            })?;

        Ok(exec_response?)
    }
}
