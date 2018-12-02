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

pub use self::color::Color;
pub use self::hum_shape::HumShape;
pub use self::physics::{
    Mass,
    Velocity,
    AABB,
    Collider,
    PhysicForce,
};

impl Component for Color {
    type Storage = BTreeStorage<Self>;
}

impl Component for HumShape {
    type Storage = BTreeStorage<Self>;
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
