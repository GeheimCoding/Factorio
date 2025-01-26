#[derive(Debug, serde::Deserialize)]
pub struct ArtilleryTriggerDelivery {
    base_: crate::types::TriggerDeliveryItem,
    #[serde(default = "default_direction_deviation")]
    direction_deviation: f32,
    projectile: crate::types::EntityID,
    #[serde(default = "default_range_deviation")]
    range_deviation: f32,
    starting_speed: f32,
    #[serde(default = "default_starting_speed_deviation")]
    starting_speed_deviation: f32,
    #[serde(default = "default_trigger_fired_artillery")]
    trigger_fired_artillery: bool,
    #[serde(rename = "type")]
    type_: String,
}
fn default_direction_deviation() -> f32 {
    0.0
}
fn default_range_deviation() -> f32 {
    0.0
}
fn default_starting_speed_deviation() -> f32 {
    0.0
}
fn default_trigger_fired_artillery() -> bool {
    false
}
