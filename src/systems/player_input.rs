use fnv::FnvHashSet;
use amethyst::{
    ecs::prelude::*,
    input::InputHandler,
};
use log::*;

use crate::{
    resources::{
        Message,
        MessageChannel,
    }
};


/// Manages player inputs.
#[derive(Default)]
pub struct PlayerInput {
    currently_pressed: FnvHashSet<String>,
}

impl<'s> System<'s> for PlayerInput {
    type SystemData = (
        Read<'s, InputHandler<String, String>>,
        Write<'s, MessageChannel>,
    );

    fn run(&mut self, (inputs, mut messages): Self::SystemData) {
        for action in inputs.bindings.actions() {
            let was_down = self.currently_pressed.contains(&action);
            let is_down = inputs.action_is_down(&action).unwrap_or(false);

            // Released
            if was_down && !is_down {
                self.currently_pressed.remove(&action);
            }

            // Pressed
            if !was_down && is_down {
                if let Some(msg) = match action.as_ref() {
                    "shift" => None,
                    "reload" => {
                        if self.currently_pressed.contains("shift") {
                            Some(Message::ReloadLevels)
                        } else {
                            Some(Message::ReloadLevel)
                        }
                    },
                    unknown => {
                        debug!("Unhandled input action: {:?}", unknown);
                        None
                    }
                } {
                    debug!("New message: {:?}", msg);
                    messages.single_write(msg);
                }
                self.currently_pressed.insert(action);
            }
        }

        for axis in inputs.bindings.axes() {
            let value = inputs.axis_value(&axis).unwrap_or(0.0);
            if value != 0.0 {
                match axis.as_ref() {
                    "camera_x" => messages.single_write(Message::CameraMoveX(value as f32)),
                    "camera_y" => messages.single_write(Message::CameraMoveY(value as f32)),
                    unknown => debug!("Unhandled input axis {} value: {}", unknown, value),
                }
            }
        }
    }
}
