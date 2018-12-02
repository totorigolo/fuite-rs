use serde_derive::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Health(f32);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dead;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoodBot;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadBot;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CurrentAction {
    None,
    Sleeping,
    GoingLeft,
    GoingRight,
}
