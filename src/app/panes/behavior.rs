use super::Pane;
use egui::{menu::bar, RichText, Ui, WidgetText};
use egui_phosphor::regular::LINK;
use egui_tiles::{Tile, TileId, Tiles, Tree, UiResponse};
use serde::{Deserialize, Serialize};

const SIZE: f32 = 16.0;

/// Behavior
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub(crate) struct Behavior {
    pub(crate) close: Option<TileId>,
    pub(crate) toggle: Option<TileId>,
}

impl Behavior {
    pub(crate) fn settings(&mut self, ui: &mut Ui, tree: &mut Tree<Pane>) {
        ui.separator();
        for tile_id in tree.active_tiles() {
            if let Some(Tile::Pane(pane)) = tree.tiles.get_mut(tile_id) {
                ui.visuals_mut().collapsing_header_frame = true;
                // let open = self
                //     .toggle
                //     .take_if(|toggle| *toggle == tile_id)
                //     .map(|tile_id| {
                //         let id = ui.make_persistent_id(tile_id);
                //         ui.data_mut(|data| {
                //             let open = data.get_persisted_mut_or_default::<bool>(id);
                //             *open = !*open;
                //             *open
                //         })
                //     });
                ui.collapsing(RichText::new(pane.title()).heading(), |ui| {
                    pane.settings(ui);
                });
            }
        }
    }
}

impl egui_tiles::Behavior<Pane> for Behavior {
    fn tab_title_for_pane(&mut self, pane: &Pane) -> WidgetText {
        pane.name().into()
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
        match pane.ui(ui) {
            Some(Event::Close) => {
                self.close = Some(tile_id);
                UiResponse::None
            }
            Some(Event::Toggle) => {
                self.toggle = Some(tile_id);
                UiResponse::None
            }
            Some(Event::Drag) => UiResponse::DragStarted,
            None => UiResponse::None,
        }
    }
}

/// Behavior settings
#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) struct Settings {
    pub(crate) link: bool,
}
