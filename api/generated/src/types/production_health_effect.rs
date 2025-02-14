#[derive(Debug, serde::Deserialize)]
pub struct ProductionHealthEffect {
    #[serde(default = "default_not_producing")]
    not_producing: f32,
    #[serde(default = "default_producing")]
    producing: f32,
}
fn default_not_producing() -> f32 {
    0.0
}
fn default_producing() -> f32 {
    0.0
}
