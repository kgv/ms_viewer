use polars::frame::DataFrame;
use serde::{Deserialize, Serialize};

/// State
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub(crate) struct State {
    pub(crate) data_frames: Vec<DataFrame>,
    pub(crate) index: usize,
}
