use self::settings::Settings;
use self::state::State;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub(crate) struct Context {
    pub(crate) state: State,
    pub(crate) settings: Settings,
}

pub(crate) mod settings;
pub(crate) mod state;
