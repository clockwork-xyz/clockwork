use crate::ordinal::{Ordinal, OrdinalSet};
use crate::time_unit::TimeUnitField;
use std::borrow::Cow;

#[derive(Clone, Debug, Eq)]
pub struct Hours {
    ordinals: Option<OrdinalSet>,
}

impl TimeUnitField for Hours {
    fn from_optional_ordinal_set(ordinal_set: Option<OrdinalSet>) -> Self {
        Hours {
            ordinals: ordinal_set,
        }
    }
    fn name() -> Cow<'static, str> {
        Cow::from("Hours")
    }
    fn inclusive_min() -> Ordinal {
        0
    }
    fn inclusive_max() -> Ordinal {
        23
    }
    fn ordinals(&self) -> OrdinalSet {
        match self.ordinals.clone() {
            Some(ordinal_set) => ordinal_set,
            None => Hours::supported_ordinals(),
        }
    }
}

impl PartialEq for Hours {
    fn eq(&self, other: &Hours) -> bool {
        self.ordinals() == other.ordinals()
    }
}
