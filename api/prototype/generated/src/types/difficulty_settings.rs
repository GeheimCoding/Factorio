#[derive(serde::Deserialize)]
pub struct DifficultySettings {
    #[serde(default = "default_spoil_time_modifier")]
    spoil_time_modifier: f64,
    #[serde(default = "default_technology_price_multiplier")]
    technology_price_multiplier: f64,
}
fn default_spoil_time_modifier() -> f64 {
    1.0
}
fn default_technology_price_multiplier() -> f64 {
    1.0
}
