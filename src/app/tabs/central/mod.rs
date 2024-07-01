use self::{plot::PlotTab, table::TableTab};
use crate::app::context::Context;
use egui::{Ui, WidgetText};
use egui_dock::{DockState, TabViewer};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display, Formatter},
    ops::{Deref, DerefMut},
};

/// Central dock
#[derive(Debug, Deserialize, Serialize)]
pub(in crate::app) struct Dock {
    state: DockState<Tab>,
}

impl Default for Dock {
    fn default() -> Self {
        Self {
            state: DockState::new(vec![Tab::Table]),
        }
    }
}

impl Deref for Dock {
    type Target = DockState<Tab>;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl DerefMut for Dock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

/// Central tab
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub(in crate::app) enum Tab {
    Plot,
    Table,
}

impl Tab {
    pub(crate) const fn sign(&self) -> &'static str {
        match self {
            Self::Plot => "📊",
            Self::Table => "T",
        }
    }

    pub(crate) const fn title(&self) -> &'static str {
        match self {
            Self::Plot => "Plot",
            Self::Table => "Table",
        }
    }
}

impl Display for Tab {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(self.title())
    }
}

/// Central tabs
#[derive(Debug)]
pub(in crate::app) struct Tabs<'a> {
    pub(in crate::app) context: &'a mut Context,
}

impl TabViewer for Tabs<'_> {
    type Tab = Tab;

    fn scroll_bars(&self, tab: &Self::Tab) -> [bool; 2] {
        match tab {
            Tab::Plot => [false, true],
            Tab::Table => [true, false],
        }
    }

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        tab.title().into()
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        match *tab {
            Tab::Plot => PlotTab::new(self.context).view(ui),
            Tab::Table => TableTab::new(self.context).view(ui),
        }
    }
}

mod plot;
mod table;
