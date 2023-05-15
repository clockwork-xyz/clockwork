use crate::{error::*, TimeUnitSpec};
use crate::ordinal::{Ordinal, OrdinalSet};
use crate::specifier::RootSpecifier;
use crate::time_unit::TimeUnitField;
use std::borrow::Cow;

#[derive(Clone, Debug, Eq)]
pub struct Months {
    ordinals: Option<OrdinalSet>,
    field: Vec<RootSpecifier>,
}

impl TimeUnitField for Months {
    fn from_optional_ordinal_set(ordinal_set: Option<OrdinalSet>, field: Vec<RootSpecifier>) -> Self {
        Months {
            ordinals: ordinal_set,
            field,
        }
    }
    fn name() -> Cow<'static, str> {
        Cow::from("Months")
    }
    fn inclusive_min() -> Ordinal {
        1
    }
    fn inclusive_max() -> Ordinal {
        12
    }
    fn ordinal_from_name(name: &str) -> Result<Ordinal, Error> {
        //TODO: Use phf crate
        let ordinal = match name.to_lowercase().as_ref() {
            "jan" | "january" => 1,
            "feb" | "february" => 2,
            "mar" | "march" => 3,
            "apr" | "april" => 4,
            "may" => 5,
            "jun" | "june" => 6,
            "jul" | "july" => 7,
            "aug" | "august" => 8,
            "sep" | "september" => 9,
            "oct" | "october" => 10,
            "nov" | "november" => 11,
            "dec" | "december" => 12,
            _ => {
                return Err(
                    ErrorKind::Expression(format!("'{}' is not a valid month name.", name)).into(),
                )
            }
        };
        Ok(ordinal)
    }

    fn name_from_ordinal(ordinal: Ordinal) -> Result<String, Error> {
        //TODO: Use phf crate
        let name = match ordinal {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => {
                return Err(
                    ErrorKind::Expression(format!("'{}' is not a valid month name.", &ordinal)).into(),
                )
            }
        };
        Ok(name.to_owned())
    }
    fn ordinals(&self) -> OrdinalSet {
        match self.ordinals.clone() {
            Some(ordinal_set) => ordinal_set,
            None => Months::supported_ordinals(),
        }
    }
    fn to_human_text (&self) ->  Result<String, Error> {
        match self.is_all() {
            true => Ok("".to_owned()),
            false => match Self::human_text_from_field(self.field.clone(), true) {
                Ok(s) => Ok(format!("in {s}")),
                Err(e) => Err(e)
            }
        }
    }
}

impl PartialEq for Months {
    fn eq(&self, other: &Months) -> bool {
        self.ordinals() == other.ordinals()
    }
}
