use serde_derive::{Serialize, Deserialize};


#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Health(pub f32);

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Dead;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Alignment {
    Good,
    Bad,
}

impl Default for Alignment {
    fn default() -> Self {
        Alignment::Good
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CurrentAction {
    None,
    Sleeping,
    GoingLeft,
    GoingRight,
    Attacking,
}

impl Default for CurrentAction {
    fn default() -> Self {
        CurrentAction::None
    }
}
