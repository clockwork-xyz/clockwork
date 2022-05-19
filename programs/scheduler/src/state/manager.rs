use {
    super::InstructionData,
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

pub const SEED_MANAGER: &[u8] = b"manager";

/**
 * Manager
 */

#[account]
#[derive(Debug)]
pub struct Manager {
    pub authority: Pubkey,
    pub queue_count: u128,
}

impl Manager {
    pub fn pda(authority: Pubkey) -> PDA {
        Pubkey::find_program_address(&[SEED_MANAGER, authority.as_ref()], &crate::ID)
    }
}

impl TryFrom<Vec<u8>> for Manager {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Manager::try_deserialize(&mut data.as_slice())
    }
}

/**
 * ManagerAccount
 */

pub trait ManagerAccount {
    fn new(&mut self, authority: Pubkey) -> Result<()>;

    fn sign(
        &self,
        account_infos: &[AccountInfo],
        bump: u8,
        ix: &InstructionData,
    ) -> Result<Option<ExecResponse>>;
}

impl ManagerAccount for Account<'_, Manager> {
    fn new(&mut self, authority: Pubkey) -> Result<()> {
        self.authority = authority;
        self.queue_count = 0;
        Ok(())
    }

    fn sign(
        &self,
        account_infos: &[AccountInfo],
        bump: u8,
        ix: &InstructionData,
    ) -> Result<Option<ExecResponse>> {
        invoke_signed(
            &Instruction::from(ix),
            account_infos,
            &[&[SEED_MANAGER, self.authority.as_ref(), &[bump]]],
        )
        .map_err(|_err| CronosError::InnerIxFailed)?;

        match get_return_data() {
            None => Ok(None),
            Some((program_id, return_data)) => {
                if program_id != ix.program_id {
                    Err(CronosError::InvalidReturnData.into())
                } else {
                    Ok(Some(
                        ExecResponse::try_from_slice(return_data.as_slice())
                            .map_err(|_err| CronosError::InvalidExecResponse)?,
                    ))
                }
            }
        }
    }
}
