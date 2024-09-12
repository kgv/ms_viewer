use self::{
    behavior::Behavior,
    settings::{Settings, Sort, TimeUnits},
};
use crate::app::MAX_PRECISION;
use egui::{ComboBox, DragValue, Ui};
use egui_phosphor::regular::{CHART_BAR, TABLE};
use egui_tiles::TileId;
use polars::frame::DataFrame;
use serde::{Deserialize, Serialize};

/// Pane
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub(crate) struct Pane {
    data_frame: DataFrame,
    settings: Settings,
    kind: Kind,
}

impl Pane {
    pub(crate) const fn icon(&self) -> &str {
        match self.kind {
            Kind::Plot => CHART_BAR,
            Kind::Table => TABLE,
        }
    }

    pub(crate) const fn title(&self) -> &'static str {
        match self.kind {
            Kind::Plot => "Plot",
            Kind::Table => "Table",
        }
    }
}

impl Pane {
    pub(crate) fn ui(&mut self, ui: &mut Ui) -> Option<Event> {}

    pub(crate) fn settings(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Retention time");
            ComboBox::from_id_source("retention_time_units")
                .selected_text(self.settings.retention_time.units.singular())
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.settings.retention_time.units,
                        TimeUnits::Millisecond,
                        TimeUnits::Millisecond.singular(),
                    )
                    .on_hover_text(TimeUnits::Millisecond.abbreviation());
                    ui.selectable_value(
                        &mut self.settings.retention_time.units,
                        TimeUnits::Second,
                        TimeUnits::Second.singular(),
                    )
                    .on_hover_text(TimeUnits::Second.abbreviation());
                    ui.selectable_value(
                        &mut self.settings.retention_time.units,
                        TimeUnits::Minute,
                        TimeUnits::Minute.singular(),
                    )
                    .on_hover_text(TimeUnits::Minute.abbreviation());
                })
                .response
                .on_hover_text(format!(
                    "Units {}",
                    self.settings.retention_time.units.abbreviation(),
                ));
            ui.add(
                DragValue::new(&mut self.settings.retention_time.precision)
                    .range(0..=MAX_PRECISION),
            )
            .on_hover_text("Precision");
        });
        ui.horizontal(|ui| {
            ui.label("Mass to charge");
            ui.add(
                DragValue::new(&mut self.settings.mass_to_charge.precision)
                    .range(0..=MAX_PRECISION),
            )
            .on_hover_text("Precision");
        });
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Explode");
            ui.checkbox(&mut self.settings.explode, "")
                .on_hover_text("Explode lists");
        });
        ui.horizontal(|ui| {
            ui.label("Filter empty/null");
            ui.checkbox(&mut self.settings.filter_null, "")
                .on_hover_text("Filter empty/null retention time");
        });
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Sort");
            ComboBox::from_id_source("sort")
                .selected_text(self.settings.sort.display())
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.settings.sort,
                        Sort::RetentionTime,
                        Sort::RetentionTime.display(),
                    )
                    .on_hover_text(Sort::RetentionTime.description());
                    ui.selectable_value(
                        &mut self.settings.sort,
                        Sort::MassToCharge,
                        Sort::MassToCharge.display(),
                    )
                    .on_hover_text(Sort::MassToCharge.description());
                })
                .response
                .on_hover_text(self.settings.sort.description());
        });
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Legend");
            ui.checkbox(&mut self.settings.legend, "")
                .on_hover_text("Show plot legend");
        });
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.settings.visible, Some(true), "◉👁");
            ui.selectable_value(&mut self.settings.visible, Some(false), "◎👁");
        });
    }
}

/// Pane kind
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) enum Kind {
    Plot,
    Table,
}

pub(crate) mod behavior;
pub(crate) mod settings;

pub(crate) mod central;
pub(crate) mod left;
