use amethyst::{
    ecs::prelude::*,
    core::timing::Time,
    ui::{UiFinder, UiText},
};
#[allow(unused_imports)]
use log::*;
use rand::{
    distributions::{
        Distribution,
        Uniform,
    },
};

use crate::{
    resources::*
};


/// Manages all game texts.
#[derive(Default)]
pub struct Text {
    name_display: Option<Entity>,
    name_display_time_left: Option<f32>,
    comment_display: Option<Entity>,
    comment_display_time_left: Option<f32>,

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

    fn run(&mut self, (time, mut ui_text, finder, message_channel, level): Self::SystemData) {
        // Getting UI entities
        if let None = self.name_display {
            if let Some(name_ui_entity) = finder.find("name") {
                self.name_display = Some(name_ui_entity);
            }
        }
        if let None = self.comment_display {
            if let Some(comment_ui_entity) = finder.find("comment") {
                self.comment_display = Some(comment_ui_entity);
            }
        }

        // Process messages in the MessageChannel
        for message in message_channel.read(self.message_reader.as_mut().unwrap()) {
            match message {
                Message::LevelStarted => {
                    let name = {
                        let idx = level.current_level.expect("current_level is None!");
                        let level = level.levels[idx].clone();
                        level.name
                    };
                    info!("Changing level name to: {}.", name);

                    if let Some(name_display) = ui_text
                        .get_mut(self.name_display.expect("Name UI text is None.")) {
                        name_display.text = name;
                        self.name_display_time_left = Some(3.0);
                    } else {
                        error!("Failed to change the level name!");
                    }
                }
                Message::DeadGoodBot | Message::DeadBadBot => {
                    let name = if *message == Message::DeadGoodBot {"A nice Hum"} else { "A bad Hum" };
                    let joke = new_joke(name);
                    info!("Joke: {}.", joke);

                    if let Some(comment_display) = ui_text
                        .get_mut(self.comment_display.expect("Comment UI text is None.")) {
                        comment_display.text = joke;
                        self.comment_display_time_left = Some(3.0);
                    } else {
                        error!("Failed to change the level comment!");
                    }
                }
                _ => {}
            }
        }

        // Update/manage display time
        if let Some(mut time_left) = self.name_display_time_left {
            time_left -= time.delta_seconds();
            if time_left <= 0.0 {
                if let Some(name_display) = ui_text
                    .get_mut(self.name_display.expect("Name UI text is None.")) {
                    name_display.text = "".into();
                    self.name_display_time_left = None;
                }
            } else {
                self.name_display_time_left = Some(time_left);
            }
        }
        if let Some(mut time_left) = self.comment_display_time_left {
            time_left -= time.delta_seconds();
            if time_left <= 0.0 {
                if let Some(comment_display) = ui_text
                    .get_mut(self.comment_display.expect("Comment UI text is None.")) {
                    comment_display.text = "".into();
                    self.comment_display_time_left = None;
                }
            } else {
                self.comment_display_time_left = Some(time_left);
            }
        }
    }
}

fn new_joke(name: &str) -> String {
    let dist = Uniform::new_inclusive(0, 5);
    let idx = dist.sample(&mut rand::thread_rng());

    match idx {
        0 => format!("{} went to meet his creator", name),
        1 => format!("{} go to space", name),
        2 => format!("{}: SSSPPPAAAAAAACCCEEEEE", name),
        3 => format!("{}: To infinity and beyond!", name),
        4 => format!("{}: Look! Look! I can fly! I can fly!", name),
        _ => format!("{}: Bye, losers!.", name),
    }.into()
}
