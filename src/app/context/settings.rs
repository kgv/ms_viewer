use serde::{Deserialize, Serialize};
use uom::si::time::{millisecond, minute, second, Units};

/// Settings
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub(crate) struct Settings {
    pub(crate) explode: bool,
    pub(crate) filter_null: bool,
    pub(crate) mass_to_charge: MassToCharge,
    pub(crate) retention_time: RetentionTime,
    pub(crate) sort: Sort,

    pub(crate) legend: bool,
    pub(crate) visible: Option<bool>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub(crate) enum Sort {
    #[default]
    RetentionTime,
    MassToCharge,
}

impl Sort {
    pub(crate) fn display(&self) -> &'static str {
        match self {
            Self::RetentionTime => "Retention time",
            Self::MassToCharge => "Mass to charge",
        }
    }

    pub(crate) fn description(&self) -> &'static str {
        match self {
            Self::RetentionTime => "Sort by retention time column",
            Self::MassToCharge => "Sort by mass to charge column",
        }
    }
}

/// Mass to charge settings
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub(crate) struct MassToCharge {
    pub(crate) precision: usize,
}

impl Default for MassToCharge {
    fn default() -> Self {
        Self { precision: 1 }
    }
}

/// Retention time settings
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub(crate) struct RetentionTime {
    pub(crate) precision: usize,
    pub(crate) units: TimeUnits,
}

impl Default for RetentionTime {
    fn default() -> Self {
        Self {
            precision: 2,
            units: Default::default(),
        }
    }
}

/// Time units
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum TimeUnits {
    Millisecond,
    #[default]
    Second,
    Minute,
}

impl TimeUnits {
    pub fn abbreviation(&self) -> &'static str {
        Units::from(*self).abbreviation()
    }

    pub fn singular(&self) -> &'static str {
        Units::from(*self).singular()
    }

    pub fn plural(&self) -> &'static str {
        Units::from(*self).plural()
    }
}

impl From<TimeUnits> for Units {
    fn from(value: TimeUnits) -> Self {
        match value {
            TimeUnits::Millisecond => Units::millisecond(millisecond),
            TimeUnits::Second => Units::second(second),
            TimeUnits::Minute => Units::minute(minute),
        }
    }
}
