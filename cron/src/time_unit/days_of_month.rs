use crate::TimeUnitSpec;
use crate::error::Error;
use crate::ordinal::{Ordinal, OrdinalSet};
use crate::specifier::RootSpecifier;
use crate::time_unit::TimeUnitField;
use std::borrow::Cow;

#[derive(Clone, Debug, Eq)]
pub struct DaysOfMonth {
    ordinals: Option<OrdinalSet>,
    field: Vec<RootSpecifier>
}

impl TimeUnitField for DaysOfMonth {
    fn from_optional_ordinal_set(ordinal_set: Option<OrdinalSet>, field: Vec<RootSpecifier>) -> Self {
        DaysOfMonth {
            ordinals: ordinal_set,
            field
        }
    }
    fn name() -> Cow<'static, str> {
        Cow::from("Days of Month")
    }
    fn inclusive_min() -> Ordinal {
        1
    }
    fn inclusive_max() -> Ordinal {
        31
    }
    fn ordinals(&self) -> OrdinalSet {
        match self.ordinals.clone() {
            Some(ordinal_set) => ordinal_set,
            None => DaysOfMonth::supported_ordinals(),
        }
    }
    fn name_in_text() -> Cow<'static, str> {
        Cow::from("day-of-month")
    }
    fn to_human_text (&self) ->  Result<String, Error> {
        match self.is_all() {
            true => Ok("".to_owned()),
            false => match Self::human_text_from_field(self.field.clone(), false) {
                Ok(s) => Ok(format!("on {s}")),
                Err(e) => Err(e)
            }
        }
    }
}

impl PartialEq for DaysOfMonth {
    fn eq(&self, other: &DaysOfMonth) -> bool {
        self.ordinals() == other.ordinals()
    }
}
