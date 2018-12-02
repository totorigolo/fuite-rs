use amethyst::{
    ecs::prelude::*,
    core::{
        Transform,
        timing::Time,
    },
};
#[allow(unused_imports)]
use log::*;

use crate::{
    components::*,
    resources::LevelResource,
};


/// Gravity and collisions
#[derive(Default)]
pub struct Physics;

impl<'s> System<'s> for Physics {
    type SystemData = (
        Read<'s, Time>,
        Write<'s, LevelResource>,
        Entities<'s>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Mass>,
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, Collider>,
        ReadStorage<'s, HumShape>,
        ReadStorage<'s, AABB>,
        WriteStorage<'s, PhysicForce>,
        ReadStorage<'s, Dead>,
        ReadStorage<'s, Rocket>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
    }

    fn run(&mut self, (
        time, level, entities, mut transforms, masses, mut velocities,
        colliders, hum_shapes, aabbs, mut forces, deads, rockets
    ): Self::SystemData) {
        let elapsed: f32 = time.delta_seconds();

        if level.current_level.is_none() { return; };
        let level = &level.levels[level.current_level.unwrap()];
        let gravity = level.gravity;

        // Apply forces
        for (entity, transform, velocity, mass, hum_shape, _)
            in (&entities, &mut transforms, &mut velocities, &masses, &hum_shapes, !&rockets).join() {

            // Sum all forces applied to the body
            let mut force = gravity
                // Air drag
                - 1.0 * velocity.0;

            // Additional forces
            if let Some(PhysicForce(ref mut additional_force)) = forces.get_mut(entity) {
                force += *additional_force;
                additional_force.x = 0.0;
                additional_force.y = 0.0;
            }

            let acceleration = force / mass.0;
            let delta_position = elapsed * (velocity.0 + elapsed * acceleration / 2.0);
            velocity.0 += elapsed * acceleration;

            // Dead => no collisions
            if deads.get(entity).is_some() {
                // no x when dead
                transform.translate_y(delta_position.y);
                continue;
            }

            // Vertical update
            // COPY-PASTA, CHANGE BOTH UPDATES!
            let vertical_nb_steps = (delta_position.y / 0.05).abs() as u32 + 1;
            let vertical_step_delta = delta_position.y / vertical_nb_steps as f32;
            let mut vertical_collide = false;
            for _ in 0..vertical_nb_steps {
                transform.translate_y(vertical_step_delta);

                // Check collisions
                for (_, static_aabb, _) in (&colliders, &aabbs, !&rockets).join() {
                    vertical_collide = true
                        && transform.translation().x + (hum_shape.base / 2.0) > static_aabb.left
                        && transform.translation().x - (hum_shape.base / 2.0) < static_aabb.right
                        && transform.translation().y < static_aabb.top
                        && transform.translation().y > static_aabb.bottom;

                    if vertical_collide {
                        transform.set_y(static_aabb.top);
                        velocity.0.y = 0.0;
                        break;
                    }
                }
                if vertical_collide { break; }
            }

            // Horizontal update
            // COPY-PASTA, CHANGE BOTH UPDATES!
            let hum_half_width = hum_shape.base.max(hum_shape.top) / 2.0;
            let horizontal_nb_steps = (delta_position.x / 0.05).abs() as u32 + 1;
            let horizontal_step_delta = delta_position.x / horizontal_nb_steps as f32;
            let mut horizontal_collide = false;
            for _ in 0..horizontal_nb_steps {
                transform.translate_x(horizontal_step_delta);

                // Check collisions
                for (_, static_aabb, _) in (&colliders, &aabbs, !&rockets).join() {
                    let vertical_compatible = true
                        && transform.translation().y < static_aabb.top
                        && transform.translation().y + hum_shape.height > static_aabb.bottom;

                    let collide_left = vertical_compatible
                        && transform.translation().x - hum_half_width < static_aabb.right
                        && transform.translation().x - hum_half_width > static_aabb.left;

                    let collide_right = vertical_compatible
                        && transform.translation().x + hum_half_width > static_aabb.left
                        && transform.translation().x + hum_half_width < static_aabb.right;

                    horizontal_collide = collide_left || collide_right;

                    if collide_left {
                        transform.set_x(static_aabb.right + hum_half_width);
                    }
                    if collide_right {
                        transform.set_x(static_aabb.left - hum_half_width);
                    }
                    if horizontal_collide {
                        velocity.0.x = 0.0;
                        break;
                    }
                }
                if horizontal_collide { break; }
            }
        }
    }
}
