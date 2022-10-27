use anchor_lang::{prelude::*, AnchorDeserialize, AnchorSerialize};
use std::fmt::{Display, Formatter};

/// The crate build information
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug)]
pub struct CrateInfo {
    /// The link to the crate spec
    pub spec: String,
    /// Arbitrary blob that can be set by developers
    pub blob: String,
}

impl Display for CrateInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "spec:{} blob:{}", self.spec, self.blob)
    }
}
