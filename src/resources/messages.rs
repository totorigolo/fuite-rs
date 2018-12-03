use amethyst::{
    core::nalgebra::Vector3,
    shrev::EventChannel
};


/// Messages sent between systems.
#[derive(Debug, PartialEq)]
pub enum Message {
    // Camera
    CameraMoveX(f32),
    CameraMoveY(f32),

    // Game
    LevelStarted,
    DeadGoodBot,
    DeadBadBot,
    NewBotInRocket(i32, i32), // (f32: rocket passengers, min_capacity)
    RocketFullEnough(i32, i32), // (f32: rocket passengers, min_capacity)
    #[allow(dead_code)]
    RocketDamaged(f32, f32), // (f32: rocket health, initial)
    RocketDestroyed,

    // Player input
    MouseLeftClick(Vector3<f32>),
    MouseRightClick(Vector3<f32>),
}

/// Channel where messages are sent.
pub type MessageChannel = EventChannel<Message>;
