use crate::TimeUnitSpec;
use crate::error::Error;
use crate::ordinal::{Ordinal, OrdinalSet};
use crate::specifier::RootSpecifier;
use crate::time_unit::TimeUnitField;
use std::borrow::Cow;

#[derive(Clone, Debug, Eq)]
pub struct Years {
    ordinals: Option<OrdinalSet>,
    field: Vec<RootSpecifier>
}

impl TimeUnitField for Years {
    fn from_optional_ordinal_set(ordinal_set: Option<OrdinalSet>, field: Vec<RootSpecifier>) -> Self {
        Years {
            ordinals: ordinal_set,
            field
        }
    }
    fn name() -> Cow<'static, str> {
        Cow::from("Years")
    }

    // TODO: Using the default impl, this will make a set w/100+ items each time "*" is used.
    // This is obviously suboptimal.
    fn inclusive_min() -> Ordinal {
        1970
    }
    fn inclusive_max() -> Ordinal {
        2100
    }
    fn ordinals(&self) -> OrdinalSet {
        match self.ordinals.clone() {
            Some(ordinal_set) => ordinal_set,
            None => Years::supported_ordinals(),
        }
    }
    fn to_human_text (&self) ->  Result<String, Error> {
        match self.is_all() {
            true => Ok("".to_owned()),
            false => match Self::human_text_from_field(self.field.clone(), false) {
                Ok(s) => Ok(format!("in {s}")),
                Err(e) => Err(e)
            }
        }
    }
}

impl PartialEq for Years {
    fn eq(&self, other: &Years) -> bool {
        self.ordinals() == other.ordinals()
    }
}
