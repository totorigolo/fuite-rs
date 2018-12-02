use serde_derive::{Serialize, Deserialize};


/// Represents the shape of a Hum.
//
//      top
//     <---->
//     ______
//    /      \    |
//   /  o  o  \   | = height
//  /__________\  |
//
//  <---------->
//      base
//
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumShape {
    pub height: f32,
    pub top: f32,
    pub base: f32,

    pub border: f32,
}
