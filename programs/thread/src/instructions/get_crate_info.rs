use {
    anchor_lang::{prelude::*, system_program},
    clockwork_utils::CrateInfo,
};

/// Accounts required for the `get_crate_info` instruction.
/// We are not using system program actually
/// But anchor does not support empty structs: https://github.com/coral-xyz/anchor/pull/1659
#[derive(Accounts)]
pub struct GetCrateInfo<'info> {
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(_ctx: Context<GetCrateInfo>) -> Result<CrateInfo> {
    let spec = format!(
        "https://github.com/clockwork-xyz/clockwork/blob/v{}/programs/thread/Cargo.toml",
        version!()
    );
    let blob = "";
    let info = CrateInfo {
        spec: spec.into(),
        blob: blob.into(),
    };
    msg!("{}", info);

    Ok(info)
}
