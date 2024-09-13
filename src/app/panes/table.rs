use super::{
    eic::ExtractedIonChromatogram,
    mass_spectrum::MassSpectrum,
    settings::{Settings, Sort, TimeUnits},
};
use crate::{
    app::computers::{TableComputed, TableKey},
    utils::ChunkedArrayExt,
};
use egui::{Direction, Layout, ScrollArea, Ui};
use egui_ext::TableRowExt;
use egui_extras::{Column, TableBuilder};
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::error;
use uom::si::{
    f32::Time,
    time::{millisecond, minute, second},
};

const COLUMN_COUNT: usize = 4;

/// Table pane
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub(crate) struct TablePane {
    pub(crate) data_frame: DataFrame,
    pub(crate) settings: Settings,
}

impl TablePane {
    pub(super) fn ui(&self, ui: &mut Ui) {
        if let Err(error) = match self.settings.sort {
            Sort::RetentionTime if !self.settings.explode => self.grouped_by_retention_time(ui),
            Sort::MassToCharge if !self.settings.explode => self.grouped_by_mass_to_charge(ui),
            _ => self.exploded(ui),
        } {
            error!(%error);
            ui.label(error.to_string());
        }
    }

    fn grouped_by_mass_to_charge(&self, ui: &mut Ui) -> PolarsResult<()> {
        let width = ui.spacing().interact_size.x;
        let height = ui.spacing().interact_size.y;
        let data_frame = ui.memory_mut(|memory| {
            memory.caches.cache::<TableComputed>().get(TableKey {
                data_frame: &self.data_frame,
                settings: &self.settings,
            })
        });
        let total_rows = data_frame.height();
        // let mass_to_charge = .cast(&DataType::UInt32)?;
        let mass_to_charge = data_frame["MassToCharge"].f32()?;
        TableBuilder::new(ui)
            .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
            .column(Column::auto_with_initial_suggestion(width))
            .columns(Column::auto(), COLUMN_COUNT - 1)
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
                    ui.heading("Retention time");
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
                    // Mass to charge
                    row.left_align_col(|ui| {
                        if let Some(value) = mass_to_charge.get(row_index) {
                            ui.label(format!(
                                "{value:.*}",
                                self.settings.mass_to_charge.precision,
                            ))
                            .on_hover_text(value.to_string());
                        } else {
                            ui.label("null");
                        }
                    });
                    // EIC
                    row.left_align_col(|ui| {
                        ui.add(ExtractedIonChromatogram {
                            data_frame: &data_frame,
                            row_index,
                            settings: self.settings,
                        });
                    });
                    // row.left_align_col(|ui| {
                    //     if let Some(retention_time_signal_series) =
                    //         retention_time_signal.get_as_series(row_index)
                    //     {
                    //         // let retention_time_series = retention_time_signal_series
                    //         //     .struct_()
                    //         //     .unwrap()
                    //         //     .field_by_name("RetentionTime")
                    //         //     .unwrap();
                    //         // let retention_time = retention_time_series.i32().unwrap();
                    //         ui.label(
                    //             retention_time
                    //                 .display(|value| {
                    //                     let time = Time::new::<millisecond>(value as _);
                    //                     let value = match self.settings.retention_time.units {
                    //                         TimeUnits::Millisecond => time.get::<millisecond>(),
                    //                         TimeUnits::Second => time.get::<second>(),
                    //                         TimeUnits::Minute => time.get::<minute>(),
                    //                     };
                    //                     format!(
                    //                         "{value:.*}",
                    //                         self.settings.retention_time.precision,
                    //                     )
                    //                 })
                    //                 .to_string(),
                    //         )
                    //         .on_hover_ui(|ui| {
                    //             if let Ok(count) =
                    //                 &data_frame["RetentionTime&Signal.Count"].get(row_index)
                    //             {
                    //                 ui.label(format!("Count: {count}"));
                    //             }
                    //             if let Ok(min) = &data_frame["RetentionTime.Min"].get(row_index) {
                    //                 ui.label(format!("Min: {min}"));
                    //             }
                    //             if let Ok(max) = &data_frame["RetentionTime.Max"].get(row_index) {
                    //                 ui.label(format!("Max: {max}"));
                    //             }
                    //         })
                    //         .context_menu(|ui| {
                    //             // if ui.button("🗐 Copy").clicked() {
                    //             //     // ui.output_mut(|output| {
                    //             //     //     output.copied_text = chunked_array.iter().join(", ")
                    //             //     // });
                    //             // };
                    //             // ui.separator();
                    //             // ScrollArea::vertical().show(ui, |ui| {
                    //             //     for value in chunked_array {
                    //             //         if let Some(value) = value {
                    //             //             let time = Time::new::<millisecond>(value as _);
                    //             //             let value = match self.settings.retention_time.units {
                    //             //                 TimeUnits::Millisecond => time.get::<millisecond>(),
                    //             //                 TimeUnits::Second => time.get::<second>(),
                    //             //                 TimeUnits::Minute => time.get::<minute>(),
                    //             //             };
                    //             //             ui.label(format!(
                    //             //                 "{value:.*}",
                    //             //                 self.settings.retention_time.precision,
                    //             //             ));
                    //             //         }
                    //             //     }
                    //             // });
                    //         });
                    //     }
                    // });
                    // Signal
                    // row.left_align_col(|ui| {
                    //     if let Some(value) = signal.get_as_series(row_index) {
                    //         ui.label(value.fmt_list()).on_hover_ui(|ui| {
                    //             if let Ok(value) = &data_frame["Signal.Count"].get(row_index) {
                    //                 ui.horizontal(|ui| {
                    //                     ui.label("Count:");
                    //                     ui.label(value.to_string());
                    //                 });
                    //             }
                    //             if let Ok(value) = &data_frame["Signal.Min"].get(row_index) {
                    //                 ui.horizontal(|ui| {
                    //                     ui.label("Min:");
                    //                     ui.label(value.to_string());
                    //                 });
                    //             }
                    //             if let Ok(value) = &data_frame["Signal.Max"].get(row_index) {
                    //                 ui.horizontal(|ui| {
                    //                     ui.label("Max:");
                    //                     ui.label(value.to_string());
                    //                 });
                    //             }
                    //             if let Ok(value) = &data_frame["Signal.Sum"].get(row_index) {
                    //                 ui.horizontal(|ui| {
                    //                     ui.label("Sum:");
                    //                     ui.label(value.to_string());
                    //                 });
                    //             }
                    //         });
                    //     }
                    // });
                });
            });
        Ok(())
    }

    fn grouped_by_retention_time(&self, ui: &mut Ui) -> PolarsResult<()> {
        let width = ui.spacing().interact_size.x;
        let height = ui.spacing().interact_size.y;
        let data_frame = ui.memory_mut(|memory| {
            memory.caches.cache::<TableComputed>().get(TableKey {
                data_frame: &self.data_frame,
                settings: &self.settings,
            })
        });
        let total_rows = data_frame.height();
        let retention_time = data_frame["RetentionTime"].i32()?;
        TableBuilder::new(ui)
            .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
            .column(Column::auto_with_initial_suggestion(width))
            .columns(Column::auto(), COLUMN_COUNT - 2)
            .auto_shrink(false)
            .striped(true)
            .header(height, |mut row| {
                row.col(|ui| {
                    ui.heading("Index");
                });
                row.col(|ui| {
                    ui.heading("Retention time");
                });
                row.col(|ui| {
                    ui.heading("Masspectrum");
                });
            })
            .body(|body| {
                body.rows(height, total_rows, |mut row| {
                    let row_index = row.index();
                    // Index
                    row.col(|ui| {
                        ui.label(row_index.to_string());
                    });
                    // Retention time
                    row.left_align_col(|ui| {
                        if let Some(value) = retention_time.get(row_index) {
                            let time = Time::new::<millisecond>(value as _);
                            let value = match self.settings.retention_time.units {
                                TimeUnits::Millisecond => time.get::<millisecond>(),
                                TimeUnits::Second => time.get::<second>(),
                                TimeUnits::Minute => time.get::<minute>(),
                            };
                            ui.label(format!(
                                "{value:.*}",
                                self.settings.retention_time.precision,
                            ))
                            .on_hover_text(value.to_string());
                        }
                    });
                    // Masspectrum
                    row.left_align_col(|ui| {
                        ui.add(MassSpectrum {
                            data_frame: &data_frame,
                            row_index,
                        });
                    });
                    // // Mass to charge
                    // row.left_align_col(|ui| {
                    //     let mass_to_charge =
                    //         mass_to_charge_signal.field_by_name("MassToCharge").unwrap();
                    //     ui.label(mass_to_charge.fmt_list()).on_hover_ui(|ui| {
                    //         if let Ok(value) = &data_frame["Masspectrum.Count"].get(row_index) {
                    //             ui.label(format!("Count: {value}"));
                    //         }
                    //         if let Ok(value) = &data_frame["MassToCharge.Min"].get(row_index) {
                    //             ui.label(format!("Min: {value}"));
                    //         }
                    //         if let Ok(value) = &data_frame["MassToCharge.Max"].get(row_index) {
                    //             ui.label(format!("Max: {value}"));
                    //         }
                    //     });
                    // });
                    // // Signal
                    // row.left_align_col(|ui| {
                    //     let signal = mass_to_charge_signal.field_by_name("Signal").unwrap();
                    //     ui.label(signal.fmt_list()).on_hover_ui(|ui| {
                    //         if let Ok(value) = &data_frame["Masspectrum.Count"].get(row_index) {
                    //             ui.label(format!("Count: {value}"));
                    //         }
                    //         if let Ok(value) = &data_frame["Signal.Min"].get(row_index) {
                    //             ui.label(format!("Min: {value}"));
                    //         }
                    //         if let Ok(value) = &data_frame["Signal.Max"].get(row_index) {
                    //             ui.label(format!("Max: {value}"));
                    //         }
                    //         if let Ok(value) = &data_frame["Signal.Sum"].get(row_index) {
                    //             ui.label(format!("Sum: {value}"));
                    //         }
                    //     });
                    // });
                });
            });
        Ok(())
    }

    fn exploded(&self, ui: &mut Ui) -> PolarsResult<()> {
        let width = ui.spacing().interact_size.x;
        let height = ui.spacing().interact_size.y;
        let data_frame = ui.memory_mut(|memory| {
            memory.caches.cache::<TableComputed>().get(TableKey {
                data_frame: &self.data_frame,
                settings: &self.settings,
            })
        });
        let total_rows = data_frame.height();
        let retention_time = data_frame["RetentionTime"].i32()?;
        let mass_to_charge = data_frame["MassToCharge"].f32()?;
        let signal = data_frame["Signal"].u16()?;
        TableBuilder::new(ui)
            .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
            .column(Column::auto_with_initial_suggestion(width))
            .columns(Column::auto(), COLUMN_COUNT - 1)
            .auto_shrink(false)
            .striped(true)
            .header(height, |mut row| {
                row.col(|ui| {
                    ui.heading("Index");
                });
                let retention_time = |ui: &mut Ui| {
                    ui.heading("Retention time");
                };
                let mass_to_charge = |ui: &mut Ui| {
                    ui.heading("Mass to charge");
                };
                match self.settings.sort {
                    Sort::RetentionTime => {
                        row.col(retention_time);
                        row.col(mass_to_charge);
                    }
                    Sort::MassToCharge => {
                        row.col(mass_to_charge);
                        row.col(retention_time);
                    }
                }
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
                    // RetentionTime & MassToCharge
                    let retention_time = |ui: &mut Ui| {
                        if let Some(value) = retention_time.get(row_index) {
                            let time = Time::new::<millisecond>(value as _);
                            let value = match self.settings.retention_time.units {
                                TimeUnits::Millisecond => time.get::<millisecond>(),
                                TimeUnits::Second => time.get::<second>(),
                                TimeUnits::Minute => time.get::<minute>(),
                            };
                            ui.label(format!(
                                "{value:.*}",
                                self.settings.retention_time.precision,
                            ))
                            .on_hover_text(format!("{value}"));
                        }
                    };
                    let mass_to_charge = |ui: &mut Ui| {
                        if let Some(value) = mass_to_charge.get(row_index) {
                            ui.label(format!(
                                "{value:.*}",
                                self.settings.mass_to_charge.precision,
                            ))
                            .on_hover_text(format!("{value}"));
                        }
                    };
                    match self.settings.sort {
                        Sort::RetentionTime => {
                            row.left_align_col(retention_time);
                            row.left_align_col(mass_to_charge);
                        }
                        Sort::MassToCharge => {
                            row.left_align_col(mass_to_charge);
                            row.left_align_col(retention_time);
                        }
                    }
                    // Signal
                    row.left_align_col(|ui| {
                        if let Some(value) = signal.get(row_index) {
                            ui.label(format!("{value}"))
                                .on_hover_text(format!("{value}"));
                        }
                    });
                });
            });
        Ok(())
    }
}
