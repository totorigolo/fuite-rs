use crate::{
    config::LevelConfig,
};

/// Level resource and info.
#[derive(Debug)]
pub struct LevelResource {
    pub state: LevelResourceState,
    pub current_level: Option<usize>,
    pub levels: Vec<LevelConfig>,
}

impl Default for LevelResource {
    fn default() -> Self {
        Self {
            state: LevelResourceState::JustCreated,
            current_level: None,
            levels: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LevelResourceState {
    JustCreated,
    ConfigLoaded,
    Loaded,
    ReloadNeeded,
}
