use amethyst::core::nalgebra::Vector2;
use serde_derive::{Serialize, Deserialize};


/// Represent the mass of physic entities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mass(pub f32);

impl Default for Mass {
    fn default() -> Self {
        Mass(1.0)
    }
}

/// Represent the velocity of physic entities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Velocity(pub Vector2<f32>);

impl Default for Velocity {
    fn default() -> Self {
        Velocity(Vector2::<f32>::new(0.0, 0.0))
    }
}

/// Represents a 2D AABB.
///
/// **Note**: for static bodies, which don't have transformation, these
/// coordinates are absolute.
///
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AABB {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32
}

/// Represents a collider, ie. a static impenetrable body for physic
/// entities.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Collider;

/// Represents the forces applied to a physic entity.
///
/// **Note**: the physic engine resets the force to 0 at each physic
/// step.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicForce(pub Vector2<f32>);

impl Default for PhysicForce {
    fn default() -> Self {
        PhysicForce(Vector2::<f32>::new(0.0, 0.0))
    }
}
