use std::convert::Into;
use serde_derive::{Serialize, Deserialize};


/// Represent the color of entities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    #[allow(dead_code)]
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { r, g, b, a }
    }
}

impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}
