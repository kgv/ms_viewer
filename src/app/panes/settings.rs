use egui::{ComboBox, DragValue, Ui};
use serde::{Deserialize, Serialize};
use uom::si::time::{millisecond, minute, second, Units};

use crate::app::MAX_PRECISION;

/// Settings
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub(crate) struct Settings {
    pub(crate) explode: bool,
    pub(crate) filter_null: bool,
    pub(crate) mass_to_charge: MassToCharge,
    pub(crate) retention_time: RetentionTime,
    pub(crate) sort: Sort,

    pub(crate) legend: bool,
    pub(crate) visible: Option<bool>,
}

impl Settings {
    pub(crate) fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Retention time");
            ComboBox::from_id_source("retention_time_units")
                .selected_text(self.retention_time.units.singular())
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.retention_time.units,
                        TimeUnits::Millisecond,
                        TimeUnits::Millisecond.singular(),
                    )
                    .on_hover_text(TimeUnits::Millisecond.abbreviation());
                    ui.selectable_value(
                        &mut self.retention_time.units,
                        TimeUnits::Second,
                        TimeUnits::Second.singular(),
                    )
                    .on_hover_text(TimeUnits::Second.abbreviation());
                    ui.selectable_value(
                        &mut self.retention_time.units,
                        TimeUnits::Minute,
                        TimeUnits::Minute.singular(),
                    )
                    .on_hover_text(TimeUnits::Minute.abbreviation());
                })
                .response
                .on_hover_text(format!(
                    "Units {}",
                    self.retention_time.units.abbreviation(),
                ));
            ui.add(DragValue::new(&mut self.retention_time.precision).range(0..=MAX_PRECISION))
                .on_hover_text("Precision");
        });
        ui.horizontal(|ui| {
            ui.label("Mass to charge");
            ui.add(DragValue::new(&mut self.mass_to_charge.precision).range(0..=MAX_PRECISION))
                .on_hover_text("Precision");
        });
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Explode");
            ui.checkbox(&mut self.explode, "")
                .on_hover_text("Explode lists");
        });
        ui.horizontal(|ui| {
            ui.label("Filter empty/null");
            ui.checkbox(&mut self.filter_null, "")
                .on_hover_text("Filter empty/null retention time");
        });
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Sort");
            ComboBox::from_id_source("sort")
                .selected_text(self.sort.display())
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.sort,
                        Sort::RetentionTime,
                        Sort::RetentionTime.display(),
                    )
                    .on_hover_text(Sort::RetentionTime.description());
                    ui.selectable_value(
                        &mut self.sort,
                        Sort::MassToCharge,
                        Sort::MassToCharge.display(),
                    )
                    .on_hover_text(Sort::MassToCharge.description());
                })
                .response
                .on_hover_text(self.sort.description());
        });
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Legend");
            ui.checkbox(&mut self.legend, "")
                .on_hover_text("Show plot legend");
        });
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.visible, Some(true), "◉👁");
            ui.selectable_value(&mut self.visible, Some(false), "◎👁");
        });
    }
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
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub(crate) struct MassToCharge {
    pub(crate) precision: usize,
}

impl Default for MassToCharge {
    fn default() -> Self {
        Self { precision: 1 }
    }
}

/// Retention time settings
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
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
