use self::{
    behavior::Behavior,
    plot::PlotPane,
    settings::{Settings, Sort, TimeUnits},
    table::TablePane,
};
use crate::app::MAX_PRECISION;
use egui::{ComboBox, DragValue, Ui};
use egui_phosphor::regular::{CHART_BAR, TABLE};
use egui_tiles::TileId;
use polars::frame::DataFrame;
use serde::{Deserialize, Serialize};

/// Pane
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) enum Pane {
    Plot(PlotPane),
    Table(TablePane),
}

impl Pane {
    pub(crate) const fn icon(&self) -> &str {
        match self {
            Self::Plot(_) => CHART_BAR,
            Self::Table(_) => TABLE,
        }
    }

    pub(crate) const fn title(&self) -> &'static str {
        match self {
            Self::Plot(_) => "Plot",
            Self::Table(_) => "Table",
        }
    }

    pub(crate) const fn data_frame(&self) -> &DataFrame {
        match self {
            Self::Plot(plot) => &plot.data_frame,
            Self::Table(table) => &table.data_frame,
        }
    }
}

impl Pane {
    pub(crate) fn ui(&mut self, ui: &mut Ui) {
        match self {
            Self::Plot(plot) => plot.ui(ui),
            Self::Table(table) => table.ui(ui),
        }
    }

    pub(crate) fn settings(&mut self, ui: &mut Ui) {
        match self {
            Self::Plot(plot) => plot.settings.ui(ui),
            Self::Table(table) => table.settings.ui(ui),
        }
    }
}

pub(crate) mod behavior;
pub(crate) mod plot;
pub(crate) mod settings;
pub(crate) mod table;
pub(crate) mod widgets;
