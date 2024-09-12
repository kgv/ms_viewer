use crate::{
    app::{
        computers::{TableComputed, TableKey},
        context::{
            settings::{Sort, TimeUnits},
            Context,
        },
    },
    utils::ChunkedArrayExt,
};
use egui::{Direction, Layout, ScrollArea, Ui};
use egui_ext::TableRowExt;
use egui_extras::{Column, TableBuilder};
use polars::error::PolarsResult;
use tracing::error;
use uom::si::{
    f32::Time,
    time::{millisecond, minute, second},
};

const COLUMN_COUNT: usize = 3;

/// Table pane
pub(super) struct Table<'a> {
    pub(super) context: &'a mut Context,
}

impl<'a> Table<'a> {
    pub(super) fn new(context: &'a mut Context) -> Self {
        Self { context }
    }
}

impl Table<'_> {
    pub(super) fn view(self, ui: &mut Ui) {
        if let Err(error) = match self.context.settings.sort {
            Sort::RetentionTime if !self.context.settings.explode => {
                self.grouped_by_retention_time(ui)
            }
            Sort::MassToCharge if !self.context.settings.explode => {
                self.grouped_by_mass_to_charge(ui)
            }
            _ => self.exploded(ui),
        } {
            error!(%error);
            ui.label(error.to_string());
        }
    }

    fn grouped_by_mass_to_charge(self, ui: &mut Ui) -> PolarsResult<()> {
        let Self { context } = self;
        let height = ui.spacing().interact_size.y;
        let data_frame = ui.memory_mut(|memory| {
            memory
                .caches
                .cache::<TableComputed>()
                .get(TableKey { context })
        });
        let total_rows = data_frame.height();
        let mass_to_charge = data_frame["MassToCharge"].f32()?;
        let retention_time = data_frame["RetentionTime"].list()?;
        let signal = data_frame["Signal"].list()?;
        TableBuilder::new(ui)
            .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
            .columns(Column::auto(), COLUMN_COUNT)
            .auto_shrink(false)
            .striped(true)
            .header(height, |mut row| {
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
                    row.left_align_col(|ui| {
                        if let Some(value) = mass_to_charge.get(row_index) {
                            ui.label(format!(
                                "{value:.*}",
                                context.settings.mass_to_charge.precision,
                            ))
                            .on_hover_text(value.to_string());
                        } else {
                            ui.label("null");
                        }
                    });
                    row.left_align_col(|ui| {
                        if let Some(value) = retention_time.get_as_series(row_index) {
                            let chunked_array = value.i32().unwrap();
                            ui.label(
                                chunked_array
                                    .display(|value| {
                                        let time = Time::new::<millisecond>(value as _);
                                        let value = match context.settings.retention_time.units {
                                            TimeUnits::Millisecond => time.get::<millisecond>(),
                                            TimeUnits::Second => time.get::<second>(),
                                            TimeUnits::Minute => time.get::<minute>(),
                                        };
                                        format!(
                                            "{value:.*}",
                                            context.settings.retention_time.precision,
                                        )
                                    })
                                    .to_string(),
                            )
                            .on_hover_ui(|ui| {
                                if let Ok(value) = &data_frame["RetentionTime.Count"].get(row_index)
                                {
                                    ui.horizontal(|ui| {
                                        ui.label("Count:");
                                        ui.label(format!("{value}"));
                                    });
                                }
                                if let Ok(value) = &data_frame["RetentionTime.Min"].get(row_index) {
                                    ui.horizontal(|ui| {
                                        ui.label("Min:");
                                        ui.label(format!("{value}"));
                                    });
                                }
                                if let Ok(value) = &data_frame["RetentionTime.Max"].get(row_index) {
                                    ui.horizontal(|ui| {
                                        ui.label("Max:");
                                        ui.label(format!("{value}"));
                                    });
                                }
                            })
                            .context_menu(|ui| {
                                if ui.button("🗐 Copy").clicked() {
                                    ui.output_mut(|output| {
                                        output.copied_text = chunked_array.iter().join(", ")
                                    });
                                };
                                ui.separator();
                                ScrollArea::vertical().show(ui, |ui| {
                                    for value in chunked_array {
                                        if let Some(value) = value {
                                            let time = Time::new::<millisecond>(value as _);
                                            let value = match context.settings.retention_time.units
                                            {
                                                TimeUnits::Millisecond => time.get::<millisecond>(),
                                                TimeUnits::Second => time.get::<second>(),
                                                TimeUnits::Minute => time.get::<minute>(),
                                            };
                                            ui.label(format!(
                                                "{value:.*}",
                                                context.settings.retention_time.precision,
                                            ));
                                        }
                                    }
                                });
                            });
                        }
                    });
                    row.left_align_col(|ui| {
                        if let Some(value) = signal.get_as_series(row_index) {
                            ui.label(value.fmt_list()).on_hover_ui(|ui| {
                                if let Ok(value) = &data_frame["Signal.Count"].get(row_index) {
                                    ui.horizontal(|ui| {
                                        ui.label("Count:");
                                        ui.label(value.to_string());
                                    });
                                }
                                if let Ok(value) = &data_frame["Signal.Min"].get(row_index) {
                                    ui.horizontal(|ui| {
                                        ui.label("Min:");
                                        ui.label(value.to_string());
                                    });
                                }
                                if let Ok(value) = &data_frame["Signal.Max"].get(row_index) {
                                    ui.horizontal(|ui| {
                                        ui.label("Max:");
                                        ui.label(value.to_string());
                                    });
                                }
                                if let Ok(value) = &data_frame["Signal.Sum"].get(row_index) {
                                    ui.horizontal(|ui| {
                                        ui.label("Sum:");
                                        ui.label(value.to_string());
                                    });
                                }
                            });
                        }
                    });
                });
            });
        Ok(())
    }

    fn grouped_by_retention_time(self, ui: &mut Ui) -> PolarsResult<()> {
        let Self { context } = self;
        let height = ui.spacing().interact_size.y;
        let data_frame = ui.memory_mut(|memory| {
            memory
                .caches
                .cache::<TableComputed>()
                .get(TableKey { context })
        });
        let total_rows = data_frame.height();
        let retention_time = data_frame["RetentionTime"].i32()?;
        let mass_to_charge = data_frame["MassToCharge"].list()?;
        let signal = data_frame["Signal"].list()?;
        TableBuilder::new(ui)
            .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
            .columns(Column::auto(), COLUMN_COUNT)
            .auto_shrink(false)
            .striped(true)
            .header(height, |mut row| {
                row.col(|ui| {
                    ui.heading(format!(
                        "Retention time ({})",
                        context.settings.retention_time.units.abbreviation(),
                    ));
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
                    // Retention time
                    row.left_align_col(|ui| {
                        if let Some(value) = retention_time.get(row_index) {
                            let time = Time::new::<millisecond>(value as _);
                            let value = match context.settings.retention_time.units {
                                TimeUnits::Millisecond => time.get::<millisecond>(),
                                TimeUnits::Second => time.get::<second>(),
                                TimeUnits::Minute => time.get::<minute>(),
                            };
                            ui.label(format!(
                                "{value:.*}",
                                context.settings.retention_time.precision,
                            ))
                            .on_hover_text(value.to_string());
                        }
                    });
                    // Mass to charge
                    row.left_align_col(|ui| {
                        if let Some(value) = mass_to_charge.get_as_series(row_index) {
                            ui.label(
                                value
                                    .f32()
                                    .unwrap()
                                    .display(|value| {
                                        format!(
                                            "{value:.*}",
                                            context.settings.mass_to_charge.precision,
                                        )
                                    })
                                    .to_string(),
                            )
                            .on_hover_ui(|ui| {
                                if let Ok(value) = &data_frame["MassToCharge.Count"].get(row_index)
                                {
                                    ui.horizontal(|ui| {
                                        ui.label("Count:");
                                        ui.label(format!("{value}"));
                                    });
                                }
                                if let Ok(value) = &data_frame["MassToCharge.Min"].get(row_index) {
                                    ui.horizontal(|ui| {
                                        ui.label("Min:");
                                        ui.label(format!("{value}"));
                                    });
                                }
                                if let Ok(value) = &data_frame["MassToCharge.Max"].get(row_index) {
                                    ui.horizontal(|ui| {
                                        ui.label("Max:");
                                        ui.label(format!("{value}"));
                                    });
                                }
                            });
                        }
                    });
                    // Signal
                    row.left_align_col(|ui| {
                        if let Some(value) = signal.get_as_series(row_index) {
                            ui.label(value.fmt_list()).on_hover_ui(|ui| {
                                if let Ok(value) = &data_frame["Signal.Count"].get(row_index) {
                                    ui.horizontal(|ui| {
                                        ui.label("Count:");
                                        ui.label(format!("{value}"));
                                    });
                                }
                                if let Ok(value) = &data_frame["Signal.Min"].get(row_index) {
                                    ui.horizontal(|ui| {
                                        ui.label("Min:");
                                        ui.label(format!("{value}"));
                                    });
                                }
                                if let Ok(value) = &data_frame["Signal.Max"].get(row_index) {
                                    ui.horizontal(|ui| {
                                        ui.label("Max:");
                                        ui.label(format!("{value}"));
                                    });
                                }
                                if let Ok(value) = &data_frame["Signal.Sum"].get(row_index) {
                                    ui.horizontal(|ui| {
                                        ui.label("Sum:");
                                        ui.label(format!("{value}"));
                                    });
                                }
                            });
                        }
                    });
                });
            });
        Ok(())
    }

    fn exploded(self, ui: &mut Ui) -> PolarsResult<()> {
        let Self { context } = self;
        let height = ui.spacing().interact_size.y;
        let data_frame = ui.memory_mut(|memory| {
            memory
                .caches
                .cache::<TableComputed>()
                .get(TableKey { context })
        });
        let total_rows = data_frame.height();
        let retention_time = data_frame["RetentionTime"].i32()?;
        let mass_to_charge = data_frame["MassToCharge"].f32()?;
        let signal = data_frame["Signal"].u16()?;
        TableBuilder::new(ui)
            .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
            .columns(Column::auto(), COLUMN_COUNT)
            .auto_shrink(false)
            .striped(true)
            .header(height, |mut row| {
                let retention_time = |ui: &mut Ui| {
                    ui.heading(format!(
                        "Retention time ({})",
                        context.settings.retention_time.units.abbreviation(),
                    ));
                };
                let mass_to_charge = |ui: &mut Ui| {
                    ui.heading("Mass to charge");
                };
                match context.settings.sort {
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
                    let retention_time = |ui: &mut Ui| {
                        if let Some(value) = retention_time.get(row_index) {
                            let time = Time::new::<millisecond>(value as _);
                            let value = match context.settings.retention_time.units {
                                TimeUnits::Millisecond => time.get::<millisecond>(),
                                TimeUnits::Second => time.get::<second>(),
                                TimeUnits::Minute => time.get::<minute>(),
                            };
                            ui.label(format!(
                                "{value:.*}",
                                context.settings.retention_time.precision,
                            ))
                            .on_hover_text(format!("{value}"));
                        }
                    };
                    let mass_to_charge = |ui: &mut Ui| {
                        if let Some(value) = mass_to_charge.get(row_index) {
                            ui.label(format!(
                                "{value:.*}",
                                context.settings.mass_to_charge.precision,
                            ))
                            .on_hover_text(format!("{value}"));
                        }
                    };
                    match context.settings.sort {
                        Sort::RetentionTime => {
                            row.left_align_col(retention_time);
                            row.left_align_col(mass_to_charge);
                        }
                        Sort::MassToCharge => {
                            row.left_align_col(mass_to_charge);
                            row.left_align_col(retention_time);
                        }
                    }
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
