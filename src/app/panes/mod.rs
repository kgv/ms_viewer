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
        self.settings.ui(ui)
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
pub(crate) mod table;

// pub(crate) mod central;
