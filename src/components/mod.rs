use amethyst::{
    ecs::{
        Component,
        storage::{
            BTreeStorage,
            VecStorage,
            NullStorage,
        },
    },
};

pub mod color;
pub mod hum_shape;
pub mod physics;
pub mod bots;
pub mod rocket;

pub use self::color::Color;
pub use self::hum_shape::HumShape;
pub use self::physics::*;
pub use self::bots::*;
pub use self::rocket::*;

impl Component for Color {
    type Storage = VecStorage<Self>;
}

impl Component for HumShape {
    type Storage = VecStorage<Self>;
}

impl Component for Mass {
    type Storage = VecStorage<Self>;
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

impl Component for AABB {
    type Storage = VecStorage<Self>;
}

impl Component for Collider {
    type Storage = NullStorage<Self>;
}

impl Component for PhysicForce {
    type Storage = VecStorage<Self>;
}

impl Component for Health {
    type Storage = VecStorage<Self>;
}

impl Component for Dead {
    type Storage = NullStorage<Self>;
}

impl Component for Bad {
    type Storage = NullStorage<Self>;
}

impl Component for CurrentAction {
    type Storage = VecStorage<Self>;
}

impl Component for LastSeekActionChange {
    type Storage = BTreeStorage<Self>;
}

impl Component for Rocket {
    type Storage = BTreeStorage<Self>;
}

impl Component for RocketTakeOff {
    type Storage = NullStorage<Self>;
}
