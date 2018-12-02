use amethyst::{
    ecs::prelude::*,
    core::{
        Transform,
        timing::Time,
    },
};
use rand::{
    distributions::{Bernoulli, Distribution},
};
#[allow(unused_imports)]
use log::*;

use crate::{
    components::*,
};


/// Bots
#[derive(Default)]
pub struct Bots;

impl<'s> System<'s> for Bots {
    type SystemData = (
        Read<'s, Time>,
        Entities<'s>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, AABB>,
        WriteStorage<'s, PhysicForce>,
        ReadStorage<'s, Health>,
        ReadStorage<'s, Dead>,
        ReadStorage<'s, Alignment>,
        ReadStorage<'s, CurrentAction>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
    }

    fn run(&mut self, (
        time, entities, mut transforms, aabbs, mut forces,
        healths, deads, alignments, current_actions
    ): Self::SystemData) {
        let elapsed: f32 = time.delta_seconds();

        // Apply forces
        for (entity, transform) in (&entities, &mut transforms, ).join() {

        }
    }
}

/// Execute bot current actions
#[derive(Default)]
pub struct BotsActionExecutor;

impl<'s> System<'s> for BotsActionExecutor {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
        ReadStorage<'s, CurrentAction>,
        WriteStorage<'s, PhysicForce>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
    }

    fn run(&mut self, (entities, mut transforms, velocities, actions, mut forces): Self::SystemData) {
        for (entity, transform, velocity, action, force)
            in (&entities, &mut transforms, &velocities, &actions, &mut forces).join() {
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
    }
}

/// Random hops for more alive Hums
#[derive(Default)]
pub struct BotsRandomHops;

impl<'s> System<'s> for BotsRandomHops {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
        ReadStorage<'s, CurrentAction>,
        WriteStorage<'s, PhysicForce>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
    }

    fn run(&mut self, (entities, mut transforms, velocities, actions, mut forces): Self::SystemData) {
        let hop_d = Bernoulli::new(0.005);

        for (entity, transform, velocity, action, force)
            in (&entities, &mut transforms, &velocities, &actions, &mut forces).join() {
            if velocity.0.y == 0.0
                && *action == CurrentAction::None
                && hop_d.sample(&mut rand::thread_rng()){
                force.0.y += 75.0;
                debug!("Bot #{} just hopped.", entity.id());
            }
        }
    }
}
