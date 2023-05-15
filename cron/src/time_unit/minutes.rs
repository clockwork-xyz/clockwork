use crate::TimeUnitSpec;
use crate::error::Error;
use crate::ordinal::{Ordinal, OrdinalSet};
use crate::specifier::RootSpecifier;
use crate::time_unit::TimeUnitField;
use std::borrow::Cow;

#[derive(Clone, Debug, Eq)]
pub struct Minutes {
    ordinals: Option<OrdinalSet>,
    field: Vec<RootSpecifier>,
}

impl TimeUnitField for Minutes {
    fn from_optional_ordinal_set(ordinal_set: Option<OrdinalSet>, field: Vec<RootSpecifier>) -> Self {
        Minutes {
            ordinals: ordinal_set,
            field,
        }
    }
    fn name() -> Cow<'static, str> {
        Cow::from("Minutes")
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
            None => Minutes::supported_ordinals(),
        }
    }
    fn to_human_text (&self) ->  Result<String, Error> {
        match self.is_all() {
            true => Ok("".to_owned()),
            false => match Self::human_text_from_field(self.field.clone(), false) {
                Ok(s) => Ok(format!("past {s}")),
                Err(e) => Err(e)
            }
        }
    }
}

impl PartialEq for Minutes {
    fn eq(&self, other: &Minutes) -> bool {
        self.ordinals() == other.ordinals()
    }
}
