use amethyst::{
    ecs::prelude::*,
    core::{
        Transform,
        timing::Time,
        nalgebra::Vector3,
    },
    controls::FlyControlTag,
};
#[allow(unused_imports)]
use log::*;

use crate::{
    resources::{
        Message,
        MessageChannel,
    },
};


/// FPS updater
pub struct CameraMoveSystem {
    convergence_speed: f32,
    /* unit: 1/t */
    remaining_offset: Option<Vector3<f32>>,

    message_reader: Option<ReaderId<Message>>,
}

impl Default for CameraMoveSystem {
    fn default() -> Self {
        CameraMoveSystem {
            convergence_speed: 10.0,
            remaining_offset: None,
            message_reader: None,
        }
    }
}

impl<'s> System<'s> for CameraMoveSystem {
    type SystemData = (
        Read<'s, Time>,
        WriteStorage<'s, Transform>,
        Read<'s, MessageChannel>,
        ReadStorage<'s, FlyControlTag>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.message_reader = Some(res.fetch_mut::<MessageChannel>().register_reader());
    }

    fn run(&mut self, (time, mut transforms, message_channel, fly_tags): Self::SystemData) {
        // Process messages in the MessageChannel
        for message in message_channel.read(self.message_reader.as_mut().unwrap()) {
            match message {
                Message::CameraMoveX(amount) => {
                    self.update_target_position(*amount / 10.0, 0.0)
                }
                Message::CameraMoveY(amount) => {
                    self.update_target_position(0.0, *amount / 10.0)
                }
                _ => {}
            }
        }

        // Move (all) the camera(s)
        let move_ended = if let Some(ref mut remaining_offset) = self.remaining_offset {
            let elapsed: f32 = time.delta_seconds();
            let offset_x = remaining_offset.x * elapsed * self.convergence_speed;
            let offset_y = remaining_offset.y * elapsed * self.convergence_speed;
            remaining_offset.x -= offset_x;
            remaining_offset.y -= offset_y;

            for (transform, _) in (&mut transforms, &fly_tags).join() {
                transform.translate_x(offset_x);
                transform.translate_y(offset_y);
            }

            remaining_offset.norm() <= 1e-4
        } else { false };
        if move_ended {
            self.remaining_offset = None;
        }
    }
}

impl CameraMoveSystem {
    fn update_target_position(&mut self, offset_x: f32, offset_y: f32) {
        if self.remaining_offset.is_none() {
            self.remaining_offset = Some(Vector3::new(offset_x, offset_y, 0.0));
        } else if let Some(ref mut remaining_offset) = self.remaining_offset {
            remaining_offset.x += offset_x;
            remaining_offset.y += offset_y;
        }
    }
}
