use crate::app::{
    computers::{TableComputed, TableKey},
    context::{Context, Sort},
};
use egui::{emath::round_to_decimals, Align2, RichText, Ui, Vec2};
use egui_ext::color;
use egui_plot::{Bar, BarChart, Line, Plot, PlotPoint, PlotPoints, Text};
use itertools::Itertools;
use polars::error::PolarsResult;
use std::{collections::HashMap, iter::zip};

/// Central plot tab
pub(super) struct PlotTab<'a> {
    pub(super) context: &'a mut Context,
}

impl<'a> PlotTab<'a> {
    pub(super) fn new(context: &'a mut Context) -> Self {
        Self { context }
    }
}

impl PlotTab<'_> {
    pub(super) fn view(self, ui: &mut Ui) {
        match self.context.settings.sort {
            Sort::RetentionTime if !self.context.settings.explode => {
                // self.grouped_by_retention_time(ui)
            }
            Sort::MassToCharge if !self.context.settings.explode => {
                self.grouped_by_mass_to_charge(ui).unwrap()
            }
            _ => {
                // self.exploded(ui);
            }
        }
    }

    pub(super) fn grouped_by_mass_to_charge(self, ui: &mut Ui) -> PolarsResult<()> {
        let Self { context } = self;
        let data_frame = ui.memory_mut(|memory| {
            memory
                .caches
                .cache::<TableComputed>()
                .get(TableKey { context })
        });
        // let points = data_frame.height();
        let mass_to_charge = data_frame["MassToCharge"].f32()?;
        let retention_time = data_frame["RetentionTime"].list()?;
        let signal = data_frame["Signal"].list()?;
        ui.vertical_centered_justified(|ui| {
            let mut plot = Plot::new("plot")
                .y_axis_formatter(move |y, _, _| round_to_decimals(y.value, 5).to_string());
            if context.settings.legend {
                plot = plot.legend(Default::default());
            }
            plot.show(ui, |ui| {
                // let bounds = ui.plot_bounds().range_x();
                // let width = ui.plot_bounds().width();
                // tracing::error!(?width);

                // let mut charts = Vec::new();
                // for (mass_to_charge, retention_time, signal) in
                //     zip(mass_to_charge, zip(retention_time, signal)).filter_map(
                //         |(mass_to_charge, (retention_time, signal))| {
                //             Some((mass_to_charge?, retention_time?, signal?))
                //         },
                //     )
                // {
                //     let bars = zip(retention_time.i32().unwrap(), signal.i16().unwrap())
                //         .filter_map(|(retention_time, signal)| {
                //             Some(Bar::new(retention_time? as _, signal? as _))
                //         })
                //         .collect();
                //     let chart = BarChart::new(bars)
                //         .name(mass_to_charge.to_string())
                //         .stack_on(&charts.iter().collect::<Vec<_>>());
                //     charts.push(chart);
                // }
                // for chart in charts {
                //     ui.bar_chart(chart);
                // }

                // Bars
                // for (retention_time, (signal, peak)) in zip(retention_time, zip(signal, peak)) {
                //     // let mut offset = 0.0;
                //     if let (Some(retention_time), Some(signal), Some(peak)) =
                //         (retention_time, signal, peak)
                //     {
                //         if width > 10000.0 {
                //             let bar = Bar::new(retention_time as _, signal as _)
                //                 .name(retention_time.to_string());
                //             let chart = BarChart::new(vec![bar]).name(retention_time.to_string());
                //             // .color(color(retention_time as _));
                //             ui.bar_chart(chart);
                //         } else {
                //             let fields = peak.struct_().unwrap().fields();
                //             // tracing::error!(?peak);
                //             // let chart = BarChart::new(vec![bar]).name(retention_time.to_string());
                //         }
                //     }
                // }

                // for (retention_time, chunk) in &zip(retention_time, zip(mass_to_charge, signal))
                //     .chunk_by(|(retention_time, _)| *retention_time)
                // {
                //     let mut offset = 0.0;
                //     let mut bars = Vec::new();
                //     if let Some(retention_time) = retention_time {
                //         for (_, (mass_to_charge, signal)) in chunk {
                //             if let (Some(mass_to_charge), Some(signal)) = (mass_to_charge, signal) {
                //                 let bar = Bar::new(retention_time as _, signal as _)
                //                     .name(mass_to_charge.to_string())
                //                     .base_offset(offset as _);
                //                 bars.push(bar);
                //                 offset += signal;
                //             }
                //         }
                //         let chart = BarChart::new(bars).name(retention_time.to_string());
                //         // .color(color(retention_time as _));
                //         ui.bar_chart(chart);
                //     }
                // }

                // let mut iter = zip(retention_time.into_iter(), signal.into_iter());
                // while let Some((Some(x), Some(y))) = iter.next() {
                //     let bar = Bar::new(x as _, y as _).name("x");
                //     let chart = BarChart::new(vec![bar]).name(x).color(color(x as _));
                //     ui.bar_chart(chart);
                // }

                // for (key, values) in visualized {
                //     // Bars
                //     let mut offset = 0.0;
                //     let x = key.into_inner();
                //     for (name, value) in values {
                //         let mut y = value;
                //         if percent {
                //             y *= 100.0;
                //         }
                //         let bar = Bar::new(x, y).name(name).base_offset(offset);
                //         let chart = BarChart::new(vec![bar])
                //             .width(context.settings.visualization.width)
                //             .name(x)
                //             .color(color(x as _));
                //         ui.bar_chart(chart);
                //         offset += y;
                //     }
                //     // Text
                //     if context.settings.visualization.text.show
                //         && offset >= context.settings.visualization.text.min
                //     {
                //         let y = offset;
                //         let text = Text::new(
                //             PlotPoint::new(x, y),
                //             RichText::new(format!("{y:.p$}"))
                //                 .size(context.settings.visualization.text.size)
                //                 .heading(),
                //         )
                //         .name(x)
                //         .color(color(x as _))
                //         .anchor(Align2::CENTER_BOTTOM);
                //         ui.text(text);
                //     }
                // }
            });
        });
        Ok(())
    }
}
