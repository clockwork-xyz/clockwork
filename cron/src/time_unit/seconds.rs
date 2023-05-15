use crate::error::Error;
use crate::ordinal::{Ordinal, OrdinalSet};
use crate::specifier::RootSpecifier;
use crate::time_unit::TimeUnitField;
use std::borrow::Cow;

#[derive(Clone, Debug, Eq)]
pub struct Seconds {
    ordinals: Option<OrdinalSet>,
    field: Vec<RootSpecifier>
}

impl TimeUnitField for Seconds {
    fn from_optional_ordinal_set(ordinal_set: Option<OrdinalSet>, field: Vec<RootSpecifier>) -> Self {
        Seconds {
            ordinals: ordinal_set,
            field
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
    fn to_human_text (&self) ->  Result<String, Error> {
        Self::human_text_from_field(self.field.clone(), false)
    }
}

impl PartialEq for Seconds {
    fn eq(&self, other: &Seconds) -> bool {
        self.ordinals() == other.ordinals()
    }
}
