use serde_derive::{Serialize, Deserialize};

use crate::components::physics::AABB;


/// Contains information about the Rocket.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Rocket {
    pub passengers: i32,
    pub min_passengers: i32,
    pub health: f32,
    pub initial_health: f32,
    pub aabb: AABB,
}

impl Rocket {
    pub fn new(min_passengers: i32, health: f32, aabb: AABB) -> Self {
        Rocket {
            passengers: 0,
            min_passengers,
            health,
            initial_health: health,
            aabb
        }
    }
}

/// Marks a Rocket which took off.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RocketTakeOff;
