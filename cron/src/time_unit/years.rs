use {
    crate::{
        ordinal::{
            Ordinal,
            OrdinalSet,
        },
        time_unit::TimeUnitField,
    },
    std::borrow::Cow,
};

#[derive(Clone, Debug, Eq)]
pub struct Years {
    ordinals: Option<OrdinalSet>,
}

impl TimeUnitField for Years {
    fn from_optional_ordinal_set(ordinal_set: Option<OrdinalSet>) -> Self {
        Years {
            ordinals: ordinal_set,
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
}

impl PartialEq for Years {
    fn eq(&self, other: &Years) -> bool {
        self.ordinals() == other.ordinals()
    }
}
