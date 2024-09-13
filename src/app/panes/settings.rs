use std::fmt::{self, Display, Formatter};

use egui::{ComboBox, DragValue, Ui, WidgetText};
use serde::{Deserialize, Serialize};
use uom::si::{
    f32::Time,
    time::{millisecond, minute, second, Units},
};

use crate::app::MAX_PRECISION;

/// Settings
#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) struct Settings {
    pub(crate) explode: bool,
    pub(crate) filter_null: bool,
    pub(crate) mass_to_charge: MassToCharge,
    pub(crate) retention_time: RetentionTime,
    pub(crate) sort: Sort,

    pub(crate) normalize: bool,

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
                .selected_text(self.sort.text())
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.sort,
                        Sort::RetentionTime,
                        Sort::RetentionTime.text(),
                    )
                    .on_hover_text(Sort::RetentionTime.description());
                    ui.selectable_value(
                        &mut self.sort,
                        Sort::MassToCharge,
                        Sort::MassToCharge.text(),
                    )
                    .on_hover_text(Sort::MassToCharge.description());
                })
                .response
                .on_hover_text(self.sort.description());
        });
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Normalize");
            ui.checkbox(&mut self.normalize, "")
                .on_hover_text("Normalize");
        });
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Legend");
            ui.checkbox(&mut self.legend, "")
                .on_hover_text("Show plot legend");
        });
        // ui.horizontal(|ui| {
        //     ui.selectable_value(&mut self.visible, Some(true), "◉👁");
        //     ui.selectable_value(&mut self.visible, Some(false), "◎👁");
        // });
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub(crate) enum Sort {
    #[default]
    RetentionTime,
    MassToCharge,
}

impl Sort {
    pub(crate) fn text(&self) -> &'static str {
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
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) struct MassToCharge {
    pub(crate) precision: usize,
}

impl Default for MassToCharge {
    fn default() -> Self {
        Self { precision: 1 }
    }
}

impl MassToCharge {
    pub(crate) fn format(self, value: f32) -> MassToChargeFormat {
        MassToChargeFormat {
            value,
            precision: Some(self.precision),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct MassToChargeFormat {
    value: f32,
    precision: Option<usize>,
}

impl MassToChargeFormat {
    pub(crate) fn precision(self, precision: Option<usize>) -> Self {
        Self { precision, ..self }
    }
}

impl Display for MassToChargeFormat {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let value = self.value;
        if let Some(precision) = self.precision {
            write!(f, "{value:.precision$}")
        } else {
            write!(f, "{value}")
        }
    }
}

impl From<MassToChargeFormat> for WidgetText {
    fn from(value: MassToChargeFormat) -> Self {
        value.to_string().into()
    }
}

/// Retention time settings
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) struct RetentionTime {
    pub(crate) precision: usize,
    pub(crate) units: TimeUnits,
}

impl RetentionTime {
    pub(crate) fn format(self, value: f32) -> RetentionTimeFormat {
        RetentionTimeFormat {
            value,
            precision: Some(self.precision),
            units: self.units,
        }
    }
}

impl Default for RetentionTime {
    fn default() -> Self {
        Self {
            precision: 2,
            units: Default::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct RetentionTimeFormat {
    value: f32,
    precision: Option<usize>,
    units: TimeUnits,
}

impl RetentionTimeFormat {
    pub(crate) fn precision(self, precision: Option<usize>) -> Self {
        Self { precision, ..self }
    }
}

impl Display for RetentionTimeFormat {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let time = Time::new::<millisecond>(self.value as _);
        let value = match self.units {
            TimeUnits::Millisecond => time.get::<millisecond>(),
            TimeUnits::Second => time.get::<second>(),
            TimeUnits::Minute => time.get::<minute>(),
        };
        if let Some(precision) = self.precision {
            write!(f, "{value:.precision$}")
        } else {
            write!(f, "{value}")
        }
    }
}

impl From<RetentionTimeFormat> for WidgetText {
    fn from(value: RetentionTimeFormat) -> Self {
        value.to_string().into()
    }
}

/// Time units
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
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
