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
    rocket_passengers_display: Option<Entity>,
    rocket_health_display: Option<Entity>,

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
        if let None = self.rocket_passengers_display {
            if let Some(rocket_passengers_ui_entity) = finder.find("rocket_passengers") {
                self.rocket_passengers_display = Some(rocket_passengers_ui_entity);
            }
        }
        if let None = self.rocket_health_display {
            if let Some(rocket_health_ui_entity) = finder.find("rocket_health") {
                self.rocket_health_display = Some(rocket_health_ui_entity);
            }
        }

        // Process messages in the MessageChannel
        for message in message_channel.read(self.message_reader.as_mut().unwrap()) {
            match message {
                Message::LevelStarted => {
                    let (name, comment) = {
                        let idx = level.current_level.expect("current_level is None!");
                        let level = level.levels[idx].clone();
                        (level.name, level.comment)
                    };
                    info!("Changing level name to: {}.", name);
                    self.write_name(&mut ui_text, name);
                    info!("Changing level comment to: {}.", comment);
                    self.write_comment(&mut ui_text, comment);

                    self.write_rocket_passengers(&mut ui_text, "Passengers: 0/?".to_string());
                    self.write_rocket_health(&mut ui_text, "Health: N/A".to_string());
                }
                Message::DeadGoodBot | Message::DeadBadBot => {
                    let name = if *message == Message::DeadGoodBot {"A nice Hum"} else { "A bad Hum" };
                    let joke = new_joke(name);
                    info!("Joke: {}.", joke);
                    self.write_comment(&mut ui_text, joke.into());
                }
                Message::NewBotInRocket(passengers, min_capacity) => {
                    self.write_comment(&mut ui_text, "A new Hum joined the Rocket!".into());
                    self.write_rocket_passengers(&mut ui_text,
                                                 format!("Passengers: {}/{}", passengers, min_capacity));
                }
                Message::RocketFullEnough(passengers, min_capacity) => {
                    let message = {
                        let idx = level.current_level.expect("current_level is None!");
                        let level = level.levels[idx].clone();
                        level.take_off
                    };

                    self.write_comment(&mut ui_text, message);
                    self.write_rocket_passengers(&mut ui_text,
                                                 format!("Passengers: {}/{}", passengers, min_capacity));
                }
                Message::RocketDamaged(health, initial) => {
                    self.write_rocket_health(&mut ui_text, format!("Health: {}/{}", health, initial));
                }
                Message::RocketDestroyed => {
                    self.write_name(&mut ui_text, "The Rocket is dead!".into());
                    self.write_comment(&mut ui_text, "Press R".into());
                    self.write_rocket_passengers(&mut ui_text, "Passengers: DEAD".into());
                    self.write_rocket_health(&mut ui_text, "Health: KO".into());
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

impl Text {
    fn write_name(&mut self, ui_text: &mut WriteStorage<UiText>, text: String) {
        if let Some(name_display) = ui_text
            .get_mut(self.name_display.expect("Name UI text is None.")) {
            name_display.text = text;
            self.name_display_time_left = Some(3.0);
        } else {
            error!("Failed to change the level name!");
        }
    }
    fn write_comment(&mut self, ui_text: &mut WriteStorage<UiText>, text: String) {
        if let Some(comment_display) = ui_text
            .get_mut(self.comment_display.expect("Comment UI text is None.")) {
            comment_display.text = text;
            self.comment_display_time_left = Some(3.0);
        } else {
            error!("Failed to change the level comment!");
        }
    }
    fn write_rocket_passengers(&mut self, ui_text: &mut WriteStorage<UiText>, text: String) {
        if let Some(rocket_passengers_display) = ui_text
            .get_mut(self.rocket_passengers_display.expect("Rocket Passengers UI text is None.")) {
            rocket_passengers_display.text = text;
        } else {
            error!("Failed to change the level rocket_passengers!");
        }
    }
    fn write_rocket_health(&mut self, ui_text: &mut WriteStorage<UiText>, text: String) {
        if let Some(rocket_health_display) = ui_text
            .get_mut(self.rocket_health_display.expect("Rocket Health UI text is None.")) {
            rocket_health_display.text = text;
        } else {
            error!("Failed to change the level rocket_health!");
        }
    }
}

fn new_joke(name: &str) -> String {
    let dist = Uniform::new_inclusive(0, 5);
    let idx = dist.sample(&mut rand::thread_rng());

    match idx {
        0 => format!("{} went to meet his creator", name),
        1 => format!("{} learned to fly", name),
        2 => format!("{}: SSSPPPAAAAAAACCCEEEEE", name),
        3 => format!("{}: To infinity and beyond!", name),
        4 => format!("{}: Look! Look! I can fly! I can fly!", name),
        _ => format!("{}: Bye, losers!.", name),
    }.into()
}
