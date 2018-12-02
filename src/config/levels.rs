use amethyst::core::nalgebra::Vector2;
use serde_derive::{Serialize, Deserialize};

use crate::components::{
    HumShape,
    Color,
    Mass,
    AABB,
};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumConfig {
    pub position: Vector2<f32>,
    pub mass: Mass,
    pub shape: HumShape,
    pub color: Color,
    pub health: f32,
    pub is_bad: bool,
    pub is_sleeping: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumsConfig {
    pub list: Vec<HumConfig>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformConfig {
    pub aabb: AABB,
    pub color: Color,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformsConfig {
    pub list: Vec<PlatformConfig>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelConfig {
    pub name: String,
    pub gravity: Vector2<f32>,
    pub hums: HumsConfig,
    pub platforms: PlatformsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelsConfig {
    pub start_level: usize,
    pub levels: Vec<LevelConfig>,
}

impl Default for LevelsConfig {
    fn default() -> Self {
        Self {
            start_level: 0,
            levels: Vec::new(),
        }
    }
}
