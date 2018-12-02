use amethyst::shrev::EventChannel;


/// Messages sent between systems.
#[derive(Debug)]
pub enum Message {
    // Camera
    CameraMoveX(f32),
    CameraMoveY(f32),

    // Levels
    ReloadLevels,
    ReloadLevel,
    LevelLoaded,
}

/// Channel where messages are sent.
pub type MessageChannel = EventChannel<Message>;
