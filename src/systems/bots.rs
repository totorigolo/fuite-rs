use amethyst::{
    ecs::prelude::*,
    core::{
        Transform,
        timing::Time,
        nalgebra::Vector3,
    },
};
use rand::{
    distributions::{
        Distribution,
        Bernoulli,
        Uniform,
    },
};
#[allow(unused_imports)]
use log::*;

use crate::{
    components::*,
    resources::*,
};


const ATTACKING_MIN_DIST: f32 = 0.3;
const HEAVEN_DIST: f32 = 50.0;

/// Execute bot current actions
pub struct BotsActionExecutor {
    kick_x_d: Uniform<f32>,
    kick_y_d: Uniform<f32>,
}

impl Default for BotsActionExecutor {
    fn default() -> Self {
        BotsActionExecutor {
            kick_x_d: Uniform::new_inclusive(-4.0, 4.0),
            kick_y_d: Uniform::new_inclusive(0.0, 6.0),
        }
    }
}

impl<'s> System<'s> for BotsActionExecutor {
    type SystemData = (
        Entities<'s>,
        Read<'s, Time>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Health>,
        WriteStorage<'s, CurrentAction>,
        ReadStorage<'s, Bad>,
        WriteStorage<'s, PhysicForce>,
        ReadStorage<'s, Dead>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
    }

    fn run(&mut self, (entities, time, transforms, mut healths, mut actions, bads, mut forces, deads): Self::SystemData) {
        let elapsed = time.delta_seconds();

        // Moves related to actions
        for (action, force) in (&actions, &mut forces).join() {
            match action {
                CurrentAction::None | CurrentAction::Sleeping => {}
                CurrentAction::GoingLeft => {
                    force.0.x -= 2.0;
                }
                CurrentAction::GoingRight => {
                    force.0.x += 2.0;
                }
                CurrentAction::Attacking => {
                    force.0.x /= 1.5;
                }
            }
        }
        for (entity, force, _) in (&entities, &mut forces, &deads).join() { // afterlife
            if bads.get(entity).is_some() {
                force.0.y += 5.0; // anti-gravity
            } else { // Good
                force.0.y += 15.0;
            }
        }

        // Fight
        for (entity, transform, action) in (&entities, &transforms, &mut actions).join() {
            if *action == CurrentAction::Attacking {
                let position = transform.translation();

                // For each enemy entity
                let mut attacked = false;
                for (other_entity, other_transform, other_health, other_force)
                    in (&entities, &transforms, &mut healths, &mut forces).join() {
                    let enemy = entity != other_entity
                        && bads.get(entity).is_some() != bads.get(other_entity).is_some();
                    let other_position = other_transform.translation();
                    let dist = (other_position - position).norm();

                    // Nearby enemy => attack
                    if enemy && dist <= ATTACKING_MIN_DIST {
                        other_health.0 -= 20.0 * elapsed;
                        other_force.0.x += self.kick_x_d.sample(&mut rand::thread_rng());
                        other_force.0.y += self.kick_y_d.sample(&mut rand::thread_rng());

//                        debug!("{} => {} ;; health={}", entity.id(), other_entity.id(), other_health.0);

                        attacked = true;
                        break; // One at a time
                    }
                }
                if !attacked {
                    debug!("Didn't attack, becoming passive.");
                    *action = CurrentAction::None;
                }
            }
        }
    }
}

/// Random hops for more alive Hums
pub struct BotsRandomHops {
    hop_d: Bernoulli,
    hop_x_d: Uniform<f32>,
    hop_y_d: Uniform<f32>,
}

impl Default for BotsRandomHops {
    fn default() -> Self {
        BotsRandomHops {
            hop_d: Bernoulli::new(0.04),
            hop_x_d: Uniform::new_inclusive(-9.0, 9.0),
            hop_y_d: Uniform::new_inclusive(75.0, 90.0),
        }
    }
}

impl<'s> System<'s> for BotsRandomHops {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Velocity>,
        ReadStorage<'s, CurrentAction>,
        WriteStorage<'s, PhysicForce>,
        WriteStorage<'s, Dead>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
    }

    fn run(&mut self, (entities, velocities, actions, mut forces, deads): Self::SystemData) {
        for (_entity, velocity, action, force, _)
            in (&entities, &velocities, &actions, &mut forces, !&deads).join() {
            if velocity.0.y == 0.0
                && *action == CurrentAction::None
                && self.hop_d.sample(&mut rand::thread_rng()) {
                force.0.x += self.hop_x_d.sample(&mut rand::thread_rng());
                force.0.y += self.hop_y_d.sample(&mut rand::thread_rng());
//                debug!("Bot #{} just hopped.", _entity.id());
            }
        }
    }
}

/// Buries dead bots
#[derive(Default)]
pub struct BotUndertaker;

impl<'s> System<'s> for BotUndertaker {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Health>,
        WriteStorage<'s, CurrentAction>,
        WriteStorage<'s, Dead>,
        WriteStorage<'s, Bad>,
        WriteStorage<'s, Rocket>,
        Write<'s, MessageChannel>
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
    }

    fn run(&mut self, (entities, transforms, mut healths, mut actions, mut deads, bads, rockets, mut message_channel): Self::SystemData) {

        // Kill entities with negative health
        let mut deads_entities = vec![];
        for (entity, health, _, _) in (&entities, &healths, !&deads, !&rockets).join() {
            if health.0 <= 0.0 {
                deads_entities.push(entity.clone());
            };
        };
        for dead in deads_entities.iter() {
            healths.remove(*dead).expect("Failed to remove Health.");
            actions.remove(*dead).expect("Failed to remove CurrentAction.");
            deads.insert(*dead, Dead).expect("Failed to add Dead.");

            let msg = if let Some(_) = bads.get(*dead) {
                Message::DeadBadBot
            } else {
                Message::DeadGoodBot
            };

            debug!("New message: {:?}", msg);
            message_channel.single_write(msg);
        };

        // Remove far enough entities
        for (entity, transform, _) in (&entities, &transforms, &deads).join() {
            if transform.translation().norm() > HEAVEN_DIST {
                entities.delete(entity).expect("Failed to delete the dead Entity.");
            }
        }
    }
}

/// Move the bad bots
#[derive(Default)]
pub struct BadBotsMover;

impl<'s> System<'s> for BadBotsMover {
    type SystemData = (
        Entities<'s>,
        Read<'s, Time>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, CurrentAction>,
        ReadStorage<'s, Bad>,
        ReadStorage<'s, Dead>,
        ReadStorage<'s, Health>,
        ReadStorage<'s, Rocket>,
        WriteStorage<'s, LastSeekActionChange>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
    }

    // Only for bad bots
    fn run(&mut self, (entities, time, transforms, mut actions, bads, deads, healths, rockets, mut last_seek_changes): Self::SystemData) {
        let now = time.absolute_real_time_seconds();

        for (entity, transform, action, _, _) in (&entities, &transforms, &mut actions, &bads, !&deads).join() {
            let position = transform.translation();

            // For every enemy entities
            let mut nearest_enemy_entity_and_position = None;
            let mut shortest_dist = 100_000_000_000.0;
            for (other_entity, other_transform, _, _) in (&entities, &transforms, &healths, !&rockets).join() {
                let enemy = entity != other_entity
                    && bads.get(entity).is_some() != bads.get(other_entity).is_some();
                let other_position = other_transform.translation();
                let dist = (other_position - position).norm();

                // Go towards nearest enemy
                if enemy && dist < shortest_dist {
                    shortest_dist = dist;
                    nearest_enemy_entity_and_position = Some((other_entity, other_position));
                }
            }

            if let Some((_, other_pos)) = nearest_enemy_entity_and_position {

                // If attack, always possible
                let dist = (other_pos - position).norm();
                if dist < ATTACKING_MIN_DIST {
//                    debug!("Enemy is close, attacking!");
                    *action = CurrentAction::Attacking;
                } else {

                    // Otherwise move => not too often
                    if let Some(last_change_time) = last_seek_changes.get(entity) {
                        if last_change_time.time + 1.0 > now {
                            continue;
                        }
                    }
                    last_seek_changes.insert(entity, LastSeekActionChange { time: now })
                        .expect("Failed to insert LastSeekActionChange.");

                    if other_pos.x < position.x {
                        *action = CurrentAction::GoingLeft;
                    } else {
                        *action = CurrentAction::GoingRight;
                    }
                }
            } else {
                *action = CurrentAction::None;
            }
        }
    }
}

/// Absorbs nearby good bots.
#[derive(Default)]
pub struct RocketProximityDoorway;

impl<'s> System<'s> for RocketProximityDoorway {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Rocket>,
        WriteStorage<'s, RocketTakeOff>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, HumShape>,
        ReadStorage<'s, Bad>,
        ReadStorage<'s, Dead>,
        Write<'s, MessageChannel>
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
    }

    fn run(&mut self, (entities, mut rockets, mut take_offs, transforms, hum_shapes, bads, deads, mut message_channel): Self::SystemData) {
        // Fill (all the) Rocket(s)
        for (rocket_entity, rocket_transform, rocket) in (&entities, &transforms, &mut rockets).join() {
            let rocket_aabb: AABB = rocket.aabb.clone();
            let rocket_position = rocket_transform.translation();

            if rocket.health <= 0.0 {
                let msg = Message::RocketDestroyed;
                debug!("New message: {:?}", msg);
                message_channel.single_write(msg);

                continue;
            }

            for (entity, transform, hum_shape, _, _) in (&entities, &transforms, &hum_shapes, !&bads, !&deads).join() {
                let hum_shape: HumShape = hum_shape.clone();
                let hum_width = hum_shape.base.max(hum_shape.top);
                let hum_position: Vector3<f32> = *transform.translation();
                let is_in = true
                    && hum_position.x + hum_width / 2.0 > rocket_aabb.left
                    && hum_position.x - hum_width / 2.0 < rocket_aabb.right
                    && hum_position.y + hum_shape.height > rocket_position.y
                    && hum_position.y < rocket_position.y + rocket_aabb.top;

                if is_in {
                    info!("New Hum in the Rocket!");

                    entities.delete(entity)
                        .expect("Failed to remove an Entity in the Rocket.");

                    rocket.passengers += 1;

                    let msg = Message::NewBotInRocket(rocket.passengers.clone(), rocket.min_passengers);
                    debug!("New message: {:?}", msg);
                    message_channel.single_write(msg);

                    if rocket.passengers >= rocket.min_passengers {
                        take_offs.insert(rocket_entity, RocketTakeOff)
                            .expect("Failed to add the RocketTakeOff component.");

                        let msg = Message::RocketFullEnough(rocket.passengers.clone(), rocket.min_passengers);
                        debug!("New message: {:?}", msg);
                        message_channel.single_write(msg);
                    }
                }
            }
        }
    }
}

/// Vertical booster during take off.
#[derive(Default)]
pub struct RocketTakeOffSystem;

impl<'s> System<'s> for RocketTakeOffSystem {
    type SystemData = (
        Read<'s, Time>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Rocket>,
        ReadStorage<'s, RocketTakeOff>,
        Write<'s, LevelResource>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
    }

    fn run(&mut self, (time, mut transforms, rockets, take_offs, mut level): Self::SystemData) {
        let elapsed = time.delta_seconds();

        for (transform, _, _) in (&mut transforms, &rockets, &take_offs).join() {
            transform.translate_y(1.0 * elapsed);

            if transform.translation().y > 4.0 { // 4 secondes
                debug!("Next level.");
                level.finished = true;
            }
        }
    }
}

/// Action modifier with the mouse.
pub struct BotMouseAction {
    message_reader: Option<ReaderId<Message>>,
}

impl Default for BotMouseAction {
    fn default() -> Self {
        BotMouseAction {
            message_reader: None
        }
    }
}

impl<'s> System<'s> for BotMouseAction {
    type SystemData = (
        Read<'s, MessageChannel>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, HumShape>,
        WriteStorage<'s, CurrentAction>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.message_reader = Some(res.fetch_mut::<MessageChannel>().register_reader());
    }

    fn run(&mut self, (message_channel, transforms, hum_shapes, mut actions): Self::SystemData) {
        // Process messages in the MessageChannel
        for message in message_channel.read(self.message_reader.as_mut().unwrap()) {
            if let Message::MouseLeftClick(click_position) = message {

                // For the first bot under the mouse click event
                for (transform, shape, action) in (&transforms, &hum_shapes, &mut actions).join() {
                    let position = transform.translation();
                    let hum_width = shape.base.max(shape.top);

                    let collide = true
                        && click_position.x > position.x - hum_width
                        && click_position.x < position.x + hum_width
                        && click_position.y < position.y + shape.height
                        && click_position.y > position.y;

                    if collide {
                        *action = CurrentAction::GoingLeft;
                    }
                }
            }
            // OMG copy-pasta
            if let Message::MouseRightClick(click_position) = message {

                // For the first bot under the mouse click event
                for (transform, shape, action) in (&transforms, &hum_shapes, &mut actions).join() {
                    let position = transform.translation();
                    let hum_width = shape.base.max(shape.top);

                    let collide = true
                        && click_position.x > position.x - hum_width
                        && click_position.x < position.x + hum_width
                        && click_position.y < position.y + shape.height
                        && click_position.y > position.y;

                    if collide {
                        *action = CurrentAction::GoingRight;
                    }
                }
            }
        }
    }
}
