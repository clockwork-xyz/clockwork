use {
    anchor_lang::prelude::*,
    clockwork_utils::CrateInfo,
};

/// No Accounts required for the `get_crate_info` instruction.
#[derive(Accounts)]
pub struct GetCrateInfo {}

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
