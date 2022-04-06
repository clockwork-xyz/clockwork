use crate::ordinal::{Ordinal, OrdinalSet};
use crate::time_unit::TimeUnitField;
use std::borrow::Cow;

#[derive(Clone, Debug, Eq)]
pub struct Seconds {
    ordinals: Option<OrdinalSet>,
}

impl TimeUnitField for Seconds {
    fn from_optional_ordinal_set(ordinal_set: Option<OrdinalSet>) -> Self {
        Seconds {
            ordinals: ordinal_set,
        }
    }
    fn name() -> Cow<'static, str> {
        Cow::from("Seconds")
    }
    fn inclusive_min() -> Ordinal {
        0
    }
    fn inclusive_max() -> Ordinal {
        59
    }
    fn ordinals(&self) -> OrdinalSet {
        match self.ordinals.clone() {
            Some(ordinal_set) => ordinal_set,
            None => Seconds::supported_ordinals(),
        }
    }
}

impl PartialEq for Seconds {
    fn eq(&self, other: &Seconds) -> bool {
        self.ordinals() == other.ordinals()
    }
}
