use {
    super::InstructionData,
    crate::{errors::CronosError, responses::ExecResponse},
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

pub const SEED_DELEGATE: &[u8] = b"delegate";

/**
 * Delegate
 */

#[account]
#[derive(Debug)]
pub struct Delegate {
    pub authority: Pubkey,
    pub queue_count: u128,
}

impl Delegate {
    pub fn pubkey(authority: Pubkey) -> Pubkey {
        Pubkey::find_program_address(&[SEED_DELEGATE, authority.as_ref()], &crate::ID).0
    }
}

impl TryFrom<Vec<u8>> for Delegate {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Delegate::try_deserialize(&mut data.as_slice())
    }
}

/**
 * DelegateAccount
 */

pub trait DelegateAccount {
    fn new(&mut self, authority: Pubkey) -> Result<()>;

    fn sign(
        &self,
        account_infos: &[AccountInfo],
        bump: u8,
        ix: &InstructionData,
    ) -> Result<Option<ExecResponse>>;
}

impl DelegateAccount for Account<'_, Delegate> {
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
            &[&[SEED_DELEGATE, self.authority.as_ref(), &[bump]]],
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
