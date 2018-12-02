use amethyst::{
    core::nalgebra::Vector3,
    shrev::EventChannel
};


/// Messages sent between systems.
#[derive(Debug)]
pub enum Message {
    // Camera
    CameraMoveX(f32),
    CameraMoveY(f32),

    // Game
    LevelStarted,

    // Player input
    MouseLeftClick(Vector3<f32>)
}

/// Channel where messages are sent.
pub type MessageChannel = EventChannel<Message>;
