use crate::time_units::TimeUnits;
use polars::frame::DataFrame;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub(crate) struct Context {
    pub(crate) state: State,
    pub(crate) settings: Settings,
}

/// State
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub(crate) struct State {
    pub(crate) data_frames: Vec<DataFrame>,
    pub(crate) index: usize,
}

/// Settings
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub(crate) struct Settings {
    pub(crate) explode: bool,
    pub(crate) filter_null: bool,
    pub(crate) legend: bool,
    pub(crate) sort: Sort,
    pub(crate) mass_to_charge: MassToCharge,
    pub(crate) retention_time: RetentionTime,
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
