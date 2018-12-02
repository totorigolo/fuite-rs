use serde_derive::{Serialize, Deserialize};


/// Contains information about the Rocket.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rocket {
    pub min_passengers: i32,
    pub health: f32,
    pub initial_health: f32,
}
