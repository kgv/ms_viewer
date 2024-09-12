use anyhow::{Error, Result};
use egui::{Direction, Layout, Response, ScrollArea, Ui, Widget};
use egui_extras::{Column, TableBuilder};
use polars::prelude::*;

/// Masspectrum widget
pub struct Masspectrum<'a> {
    pub(crate) data_frame: &'a DataFrame,
    pub(crate) row_index: usize,
}

impl Widget for Masspectrum<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let height = ui.spacing().interact_size.y;
        let width = ui.spacing().interact_size.x;
        let masspectrum = self.data_frame["Masspectrum"].list().unwrap();
        let masspectrum_series = masspectrum.get_as_series(self.row_index).unwrap();
        ui.menu_button(masspectrum_series.fmt_list(), |ui| {
            let total_rows = masspectrum_series.len();
            let mass_to_charge_signal = masspectrum_series.struct_().unwrap();
            let mass_to_charge_series =
                mass_to_charge_signal.field_by_name("MassToCharge").unwrap();
            let signal_series = mass_to_charge_signal.field_by_name("Signal").unwrap();
            let mass_to_charge = mass_to_charge_series.f32().unwrap();
            let signal = signal_series.u16().unwrap();
            TableBuilder::new(ui)
                .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
                .column(Column::auto_with_initial_suggestion(width))
                .columns(Column::auto(), 2)
                .auto_shrink(false)
                .striped(true)
                .header(height, |mut row| {
                    row.col(|ui| {
                        ui.heading("Index");
                    });
                    row.col(|ui| {
                        ui.heading("Mass to charge");
                    });
                    row.col(|ui| {
                        ui.heading("Signal");
                    });
                })
                .body(|body| {
                    body.rows(height, total_rows, |mut row| {
                        let row_index = row.index();
                        // Index
                        row.col(|ui| {
                            ui.label(row_index.to_string());
                        });
                    });
                });

            ScrollArea::vertical().show_rows(ui, height, total_rows, |ui, row_range| {
                for row_index in row_range {
                    let text = format!(
                        "{}: {} {}",
                        row_index + 1,
                        mass_to_charge.get(row_index).unwrap(),
                        signal.get(row_index).unwrap(),
                    );
                    ui.label(text);
                }
            });
            Ok::<_, PolarsError>(())
        })
        .response
        .on_hover_ui(|ui| {
            if let Ok(value) = &self.data_frame["Masspectrum.Count"].get(self.row_index) {
                ui.label(format!("Count: {value}"));
            }
        })
        .on_hover_ui(|ui| {
            ui.heading("Mass to charge");
            if let Ok(value) = &self.data_frame["MassToCharge.Min"].get(self.row_index) {
                ui.label(format!("Min: {value}"));
            }
            if let Ok(value) = &self.data_frame["MassToCharge.Max"].get(self.row_index) {
                ui.label(format!("Max: {value}"));
            }
        })
        .on_hover_ui(|ui| {
            ui.heading("Signal");
            if let Ok(value) = &self.data_frame["Signal.Min"].get(self.row_index) {
                ui.label(format!("Min: {value}"));
            }
            if let Ok(value) = &self.data_frame["Signal.Max"].get(self.row_index) {
                ui.label(format!("Max: {value}"));
            }
            if let Ok(value) = &self.data_frame["Signal.Sum"].get(self.row_index) {
                ui.label(format!("Sum: {value}"));
            }
        })
    }
}
