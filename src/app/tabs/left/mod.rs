use crate::{
    app::context::{Context, Sort},
    time_units::TimeUnits,
};
use egui::{ComboBox, DragValue, Ui};

/// IEEE 754-2008
const MAX_PRECISION: usize = 16;

/// Settings tab
pub(crate) struct SettingsTab<'a> {
    pub(crate) context: &'a mut Context,
}

impl<'a> SettingsTab<'a> {
    pub(crate) fn new(context: &'a mut Context) -> Self {
        Self { context }
    }
}

impl SettingsTab<'_> {
    pub(crate) fn view(self, ui: &mut Ui) {
        let Self { context } = self;

        ui.horizontal(|ui| {
            ui.label("Retention time:");
            ComboBox::from_id_source("retention_time_units")
                .selected_text(context.settings.retention_time.units.singular())
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut context.settings.retention_time.units,
                        TimeUnits::Millisecond,
                        TimeUnits::Millisecond.singular(),
                    )
                    .on_hover_text(TimeUnits::Millisecond.abbreviation());
                    ui.selectable_value(
                        &mut context.settings.retention_time.units,
                        TimeUnits::Second,
                        TimeUnits::Second.singular(),
                    )
                    .on_hover_text(TimeUnits::Second.abbreviation());
                    ui.selectable_value(
                        &mut context.settings.retention_time.units,
                        TimeUnits::Minute,
                        TimeUnits::Minute.singular(),
                    )
                    .on_hover_text(TimeUnits::Minute.abbreviation());
                })
                .response
                .on_hover_text(format!(
                    "Units: {}",
                    context.settings.retention_time.units.abbreviation(),
                ));
            ui.add(
                DragValue::new(&mut context.settings.retention_time.precision)
                    .clamp_range(0..=MAX_PRECISION),
            )
            .on_hover_text("Precision");
        });
        ui.horizontal(|ui| {
            ui.label("Mass to charge:");
            ui.add(
                DragValue::new(&mut context.settings.mass_to_charge.precision)
                    .clamp_range(0..=MAX_PRECISION),
            )
            .on_hover_text("Precision");
        });
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Explode:");
            ui.checkbox(&mut context.settings.explode, "")
                .on_hover_text("Explode lists");
        });
        ui.horizontal(|ui| {
            ui.label("Filter empty/null:");
            ui.checkbox(&mut context.settings.filter_null, "")
                .on_hover_text("Filter empty/null retention time");
        });
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Sort:");
            ComboBox::from_id_source("sort")
                .selected_text(context.settings.sort.display())
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut context.settings.sort,
                        Sort::RetentionTime,
                        Sort::RetentionTime.display(),
                    )
                    .on_hover_text(Sort::RetentionTime.description());
                    ui.selectable_value(
                        &mut context.settings.sort,
                        Sort::MassToCharge,
                        Sort::MassToCharge.display(),
                    )
                    .on_hover_text(Sort::MassToCharge.description());
                })
                .response
                .on_hover_text(context.settings.sort.description());
        });
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Legend:");
            ui.checkbox(&mut context.settings.legend, "")
                .on_hover_text("Show plot legend");
        });
        ui.horizontal(|ui| {
            ui.selectable_value(&mut context.settings.visible, Some(true), "◉👁");
            ui.selectable_value(&mut context.settings.visible, Some(false), "◎👁");
        });
    }
}
