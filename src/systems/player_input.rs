use fnv::FnvHashSet;
use amethyst::{
    ecs::prelude::*,
    core::{
        Transform,
        nalgebra::Vector3,
    },
    renderer::{
        Camera,
        ScreenDimensions,
    },
    input::InputHandler,
    controls::FlyControlTag,
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
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        ReadStorage<'s, FlyControlTag>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
    }

    fn run(&mut self, (inputs, mut messages, transforms, cameras, fly_tags, screen_dim): Self::SystemData) {
        // Get the (a?) camera
        let camera_transform = (&transforms, &cameras, &fly_tags).join().next().or(None);

        // Handle discrete actions
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
                    "left_click" | "right_click" => {
                        if let Some(position) = inputs.mouse_position() {
                            if let Some((t, c, _)) = camera_transform {
                                let position = self.screen_to_2d_scene(
                                    position, t, c,
                                    &*screen_dim);

                                if action.as_ref() == "left_click".to_string() {
                                    Some(Message::MouseLeftClick(position))
                                } else {
                                    Some(Message::MouseRightClick(position))
                                }
                            } else {
                                warn!("Mouse click ignored because no camera found.");
                                None
                            }
                        } else { None }
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

        // Handle analogous axes
        for axis in inputs.bindings.axes() {
            let value = inputs.axis_value(&axis).unwrap_or(0.0);
            if value != 0.0 {
                match axis.as_ref() {
                    "camera_x" => messages.single_write(Message::CameraMoveX(value as f32)),
                    "camera_y" => messages.single_write(Message::CameraMoveY(-value as f32)),
                    unknown => debug!("Unhandled input axis {} value: {}", unknown, value),
                }
            }
        }
    }
}

impl<'s> PlayerInput {
    fn screen_to_2d_scene(&mut self,
                          position: (f64, f64),
                          camera_transform: &Transform,
                          camera: &Camera,
                          screen_dim: &ScreenDimensions,
    ) -> Vector3<f32> {
        use amethyst::core::nalgebra::Orthographic3;
        let projection = Orthographic3::from_matrix_unchecked(camera.proj);

        let screen_x = position.0 as f32;
        let screen_y = position.1 as f32;

        let scene_x = screen_x / screen_dim.width() * (projection.right() - projection.left()).abs()
            - projection.right() + camera_transform.translation().x;
        let scene_y = -screen_y / screen_dim.height() * (projection.top() - projection.bottom()).abs()
            + projection.top() + camera_transform.translation().y;

//        println!("Mouse button clicked at: {:#?}", position);
////        println!("Camera: {:#?}", (camera, camera_transform));
//        println!("{:?}", (proj.left(), proj.right(), proj.top(), proj.bottom()));
//        println!("{:?}", screen_dim);
//        println!("=> {:?}", (scene_x, scene_y));

        Vector3::new(scene_x, scene_y, 0.0)
    }
}
