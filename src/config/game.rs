use serde_derive::{Serialize, Deserialize};

use crate::{
    config::LevelsConfig,
    components::Color,
};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub levels: LevelsConfig,
    pub void_color: Color,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            levels: Default::default(),
            void_color: Color::new(0.0, 0.0, 0.0, 1.0)
        }
    }
}
