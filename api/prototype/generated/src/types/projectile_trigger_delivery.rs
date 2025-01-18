#[derive(serde::Deserialize)]
pub struct ProjectileTriggerDelivery {
    base_: crate::types::TriggerDeliveryItem,
    #[serde(default = "default_direction_deviation")]
    direction_deviation: f32,
    #[serde(default = "default_max_range")]
    max_range: f64,
    #[serde(default = "default_min_range")]
    min_range: f64,
    projectile: crate::types::EntityID,
    #[serde(default = "default_range_deviation")]
    range_deviation: f32,
    starting_speed: f32,
    #[serde(default = "default_starting_speed_deviation")]
    starting_speed_deviation: f32,
    #[serde(rename = "type")]
    type_: String,
}
fn default_direction_deviation() -> f32 {
    0.0
}
fn default_max_range() -> f64 {
    1000.0
}
fn default_min_range() -> f64 {
    0.0
}
fn default_range_deviation() -> f32 {
    0.0
}
fn default_starting_speed_deviation() -> f32 {
    0.0
}
