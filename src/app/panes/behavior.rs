use std::mem::take;

use crate::{
    app::{icon, localize},
    utils::ContainerExt,
};

use super::{plot::PlotPane, table::TablePane, Pane};
use egui::{menu::bar, CollapsingHeader, CursorIcon, RichText, Ui, WidgetText};
use egui_phosphor::regular::{CHART_BAR, LINK, TABLE, X};
use egui_tiles::{Tile, TileId, Tiles, Tree, UiResponse};
use serde::{Deserialize, Serialize};

const SIZE: f32 = 16.0;

/// Behavior
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub(crate) struct Behavior {
    pub(crate) close: Option<TileId>,
    pub(crate) click: Option<TileId>,
}

impl Behavior {
    pub(crate) fn settings(&mut self, ui: &mut Ui, tree: &mut Tree<Pane>) {
        ui.separator();
        for tile_id in tree.active_tiles() {
            if let Some(Tile::Pane(pane)) = tree.tiles.get_mut(tile_id) {
                ui.visuals_mut().collapsing_header_frame = true;
                let open = self
                    .click
                    .take_if(|toggle| *toggle == tile_id)
                    .map(|tile_id| {
                        let id = ui.make_persistent_id(tile_id);
                        ui.data_mut(|data| {
                            let open = data.get_persisted_mut_or_default::<bool>(id);
                            *open = !*open;
                            *open
                        })
                    });
                CollapsingHeader::new(RichText::new(pane.title()).heading())
                    .open(open)
                    .show(ui, |ui| {
                        let text = match pane {
                            Pane::Plot(_) => TABLE,
                            Pane::Table(_) => CHART_BAR,
                        };
                        if ui
                            .button(icon!(text).size(16.0))
                            .on_hover_text(localize!("table"))
                            .clicked()
                        {
                            *pane = match pane {
                                Pane::Plot(PlotPane {
                                    data_frame,
                                    settings,
                                }) => Pane::Table(TablePane {
                                    data_frame: data_frame.clone(),
                                    settings: *settings,
                                }),
                                Pane::Table(TablePane {
                                    data_frame,
                                    settings,
                                }) => Pane::Plot(PlotPane {
                                    data_frame: data_frame.clone(),
                                    settings: *settings,
                                }),
                            };
                            // if let Some(id) = self.tree.iter {
                            //     // if let Some(Tile::Container(container)) = self.tree.tiles.get_mut(id) {
                            //     //     container.set_kind(ContainerKind::Tabs);
                            //     // }
                            // }
                        }
                        pane.settings(ui);
                    });
            }
        }
    }
}

impl egui_tiles::Behavior<Pane> for Behavior {
    fn tab_title_for_pane(&mut self, pane: &Pane) -> WidgetText {
        pane.title().into()
    }

    fn tab_title_for_tile(&mut self, tiles: &Tiles<Pane>, tile_id: TileId) -> WidgetText {
        if let Some(tile) = tiles.get(tile_id) {
            match tile {
                Tile::Pane(pane) => self.tab_title_for_pane(pane),
                Tile::Container(container) => {
                    if let Some(pane) = container.find_child_pane(tiles) {
                        format!("{}, ...", self.tab_title_for_pane(pane).text()).into()
                    } else {
                        format!("{:?}", container.kind()).into()
                    }
                }
            }
        } else {
            "MISSING TILE".into()
        }
    }

    fn pane_ui(&mut self, ui: &mut Ui, tile_id: TileId, pane: &mut Pane) -> UiResponse {
        let response = ui
            .horizontal(|ui| {
                let response = ui.heading(pane.title()).on_hover_cursor(CursorIcon::Grab);
                ui.add_space(ui.available_width() - ui.spacing().button_padding.x - SIZE);
                ui.visuals_mut().button_frame = false;
                if ui.button(RichText::new(X)).clicked() {
                    self.close = Some(tile_id);
                }
                response
            })
            .inner;
        if response.clicked() {
            self.click = Some(tile_id);
        }
        pane.ui(ui);
        if response.dragged() {
            UiResponse::DragStarted
        } else {
            UiResponse::None
        }
    }
}
