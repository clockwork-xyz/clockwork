use crate::{error::*, TimeUnitSpec};
use crate::ordinal::{Ordinal, OrdinalSet};
use crate::specifier::RootSpecifier;
use crate::time_unit::TimeUnitField;
use std::borrow::Cow;

#[derive(Clone, Debug, Eq)]
pub struct DaysOfWeek {
    ordinals: Option<OrdinalSet>,
    field: Vec<RootSpecifier>
}

impl TimeUnitField for DaysOfWeek {
    fn from_optional_ordinal_set(ordinal_set: Option<OrdinalSet>, field: Vec<RootSpecifier>) -> Self {
        DaysOfWeek {
            ordinals: ordinal_set,
            field
        }
    }
    fn name() -> Cow<'static, str> {
        Cow::from("Days of Week")
    }
    fn inclusive_min() -> Ordinal {
        1
    }
    fn inclusive_max() -> Ordinal {
        7
    }
    fn ordinal_from_name(name: &str) -> Result<Ordinal, Error> {
        //TODO: Use phf crate
        let ordinal = match name.to_lowercase().as_ref() {
            "sun" | "sunday" => 1,
            "mon" | "monday" => 2,
            "tue" | "tues" | "tuesday" => 3,
            "wed" | "wednesday" => 4,
            "thu" | "thurs" | "thursday" => 5,
            "fri" | "friday" => 6,
            "sat" | "saturday" => 7,
            _ => {
                return Err(ErrorKind::Expression(format!(
                    "'{}' is not a valid day of the week.",
                    name
                ))
                .into())
            }
        };
        Ok(ordinal)
    }
    fn name_from_ordinal(ordinal: Ordinal) -> Result<String, Error> {
        //TODO: Use phf crate
        let name = match ordinal {
            1 => "Sunday",
            2 => "Monday",
            3 => "Tuesday",
            4 => "Wednesday",
            5 => "Thursday",
            6 => "Friday",
            7 => "Saturday",
            _ => {
                return Err(ErrorKind::Expression(format!(
                    "'{}' is not a valid day of the week.",
                    ordinal
                ))
                .into())
            }
        };
        Ok(name.to_owned())
    }
    fn ordinals(&self) -> OrdinalSet {
        match self.ordinals.clone() {
            Some(ordinal_set) => ordinal_set,
            None => DaysOfWeek::supported_ordinals(),
        }
    }
    fn to_human_text (&self) ->  Result<String, Error> {
        match self.is_all() {
            true => Ok("".to_owned()),
            false => match Self::human_text_from_field(self.field.clone(), true) {
                Ok(s) => Ok(format!("on {s}")),
                Err(e) => Err(e)
            }
        }
    }
}

impl PartialEq for DaysOfWeek {
    fn eq(&self, other: &DaysOfWeek) -> bool {
        self.ordinals() == other.ordinals()
    }
}
