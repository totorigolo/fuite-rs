use amethyst::{
    ecs::prelude::*,
    core::timing::Time,
    ui::{UiFinder, UiText},
};
#[allow(unused_imports)]
use log::*;

use crate::{
    resources::*
};


/// Manages all game texts.
#[derive(Default)]
pub struct Text {
    name_display: Option<Entity>,

    message_reader: Option<ReaderId<Message>>,
}

impl<'s> System<'s> for Text {
    type SystemData = (
        Read<'s, Time>,
        WriteStorage<'s, UiText>,
        UiFinder<'s>,
        Read<'s, MessageChannel>,
        Read<'s, LevelResource>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.message_reader = Some(res.fetch_mut::<MessageChannel>().register_reader());
    }

    fn run(&mut self, (_time, mut ui_text, finder, message_channel, level): Self::SystemData) {
        // Getting UI entities
        if let None = self.name_display {
            if let Some(name_ui_entity) = finder.find("name") {
                self.name_display = Some(name_ui_entity);
            }
        }

        // Process messages in the MessageChannel
        for message in message_channel.read(self.message_reader.as_mut().unwrap()) {
            match message {
                Message::LevelLoaded => {
                    let name = {
                        let idx = level.current_level.expect("current_level is None!");
                        let level = level.levels[idx].clone();
                        level.name
                    };
                    info!("Changing level name to: {}.", name);

                    if let Some(name_display) = ui_text
                        .get_mut(self.name_display.expect("Name UI text is None.")) {
                        name_display.text = name;
                    } else {
                        error!("Failed to change the level name!");
                    }
                }
                _ => {}
            }
        }
    }
}
