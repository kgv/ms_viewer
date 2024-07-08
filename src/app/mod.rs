use self::{
    context::Context,
    tabs::central::{Tab as CentralTab, Tabs as CentralTabs},
};
use anyhow::Result;
use egui::{
    global_dark_light_mode_switch, menu::bar, warn_if_debug_build, Align, Align2, Button,
    CentralPanel, Color32, DroppedFile, Id, LayerId, Layout, Order, RichText, SidePanel, TextStyle,
    TopBottomPanel,
};
use egui_dock::{DockArea, Style};
use egui_ext::{DroppedFileExt, HoveredFileExt, WithVisuals};
use polars::frame::DataFrame;
use serde::{Deserialize, Serialize};
use std::{fmt::Write, str, time::Duration};
use tabs::{central::Dock, left::SettingsTab};
use tracing::{error, info, trace};

const _NOTIFICATIONS_DURATION: Duration = Duration::from_secs(15);

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Default, Deserialize, Serialize)]
#[serde(default)]
pub struct App {
    // Context
    context: Context,
    // Docks
    dock: Dock,
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn drag_and_drop(&mut self, ctx: &egui::Context) {
        // Preview hovering files
        if let Some(text) = ctx.input(|input| {
            (!input.raw.hovered_files.is_empty()).then(|| {
                let mut text = String::from("Dropping files:");
                for file in &input.raw.hovered_files {
                    write!(text, "\n{}", file.display()).ok();
                }
                text
            })
        }) {
            let painter =
                ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));
            let screen_rect = ctx.screen_rect();
            painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
            painter.text(
                screen_rect.center(),
                Align2::CENTER_CENTER,
                text,
                TextStyle::Heading.resolve(&ctx.style()),
                Color32::WHITE,
            );
        }
        // Parse dropped files
        if let Some(dropped_files) = ctx.input(|input| {
            (!input.raw.dropped_files.is_empty()).then_some(input.raw.dropped_files.clone())
        }) {
            info!(?dropped_files);
            // self.docks.left.tabs.files = Files {
            //     files,
            //     ..self.docks.left.tabs.files
            // };
            // ctx.data_mut(|data| data.remove_by_type::<TomlParsed>());
            for dropped in dropped_files {
                // let data_frame: DataFrame = match dropped.extension().and_then(OsStr::to_str) {
                //     Some("bin") => bincode::deserialize(&fs::read(&args.path)?)?,
                //     Some("ron") => ron::de::from_str(&fs::read_to_string(&args.path)?)?,
                //     _ => panic!("unsupported input file extension"),
                // };
                match bin(&dropped) {
                    Ok(data_frame) => {
                        trace!(?data_frame);
                        self.context.state.data_frames.push(data_frame);
                    }
                    Err(error) => {
                        error!(%error);
                        // self.toasts
                        //     .error(format!("{}: {error}", dropped.display()))
                        //     .set_closable(true)
                        //     .set_duration(Some(NOTIFICATIONS_DURATION));
                        continue;
                    }
                };
            }
        }
    }
}

impl App {
    fn panels(&mut self, ctx: &egui::Context) {
        self.top_panel(ctx);
        self.bottom_panel(ctx);
        self.left_panel(ctx);
        self.central_panel(ctx);
    }

    // Bottom panel
    fn bottom_panel(&mut self, ctx: &egui::Context) {
        TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                warn_if_debug_build(ui);
                ui.label(RichText::new(env!("CARGO_PKG_VERSION")).small());
                ui.separator();
            });
        });
    }

    // Central panel
    fn central_panel(&mut self, ctx: &egui::Context) {
        CentralPanel::default()
            .frame(egui::Frame::central_panel(&ctx.style()).inner_margin(0.0))
            .show(ctx, |ui| {
                if self.context.state.data_frames.is_empty() {
                    return;
                }
                DockArea::new(&mut self.dock)
                    .id(Id::new("central_dock"))
                    .style(Style::from_egui(&ctx.style()))
                    .show_inside(
                        ui,
                        &mut CentralTabs {
                            context: &mut self.context,
                        },
                    );
            });
    }

    // Left panel
    fn left_panel(&mut self, ctx: &egui::Context) {
        SidePanel::left("left_panel")
            .frame(egui::Frame::side_top_panel(&ctx.style()))
            .resizable(true)
            .show_animated(ctx, true, |ui| SettingsTab::new(&mut self.context).view(ui));
    }

    // Top panel
    fn top_panel(&mut self, ctx: &egui::Context) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            bar(ui, |ui| {
                ui.visuals_mut().button_frame = false;
                global_dark_light_mode_switch(ui);
                ui.separator();
                if ui
                    .add(Button::new(RichText::new("🗑")))
                    .on_hover_text("Reset data")
                    .clicked()
                {
                    *self = Default::default();
                }
                // Reset gui
                if ui
                    .add(Button::new(RichText::new("🔃")))
                    .on_hover_text("Reset gui")
                    .clicked()
                {
                    ui.with_visuals(|ui, _| ui.memory_mut(|memory| *memory = Default::default()));
                }
                // Organize windows
                if ui
                    .add(Button::new(RichText::new("▣")))
                    .on_hover_text("Organize windows")
                    .clicked()
                {
                    ui.ctx().memory_mut(|memory| memory.reset_areas());
                }
                ui.separator();
                let mut central_tab = |tab| {
                    let found = self.dock.find_tab(&tab);
                    if ui
                        .selectable_label(found.is_some(), tab.sign())
                        .on_hover_text(tab.to_string())
                        .clicked()
                    {
                        if let Some(index) = found {
                            self.dock.remove_tab(index);
                        } else {
                            self.dock.push_to_focused_leaf(tab);
                        }
                    }
                };
                // Table
                central_tab(CentralTab::Table);
                // Plot
                central_tab(CentralTab::Plot);
            });
        });
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.panels(ctx);
        self.drag_and_drop(ctx);
    }
}

fn bin(dropped_file: &DroppedFile) -> Result<DataFrame> {
    Ok(bincode::deserialize(&dropped_file.bytes()?)?)
}

mod computers;
mod context;
mod tabs;
