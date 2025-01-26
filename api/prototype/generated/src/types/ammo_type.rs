#[derive(Debug, serde::Deserialize)]
pub struct AmmoType {
    action: Option<crate::types::Trigger>,
    #[serde(default = "default_clamp_position")]
    clamp_position: bool,
    #[serde(default = "default_consumption_modifier")]
    consumption_modifier: f32,
    #[serde(default = "default_cooldown_modifier")]
    cooldown_modifier: f64,
    energy_consumption: Option<crate::types::Energy>,
    #[serde(default = "default_range_modifier")]
    range_modifier: f64,
    source_type: Option<crate::types::AmmoSourceType>,
    target_filter: Option<Vec<crate::types::EntityID>>,
    #[serde(default = "default_target_type")]
    target_type: AmmoTypeTargetType,
}
fn default_clamp_position() -> bool {
    false
}
fn default_consumption_modifier() -> f32 {
    1.0
}
fn default_cooldown_modifier() -> f64 {
    1.0
}
fn default_range_modifier() -> f64 {
    1.0
}
#[derive(Debug, serde::Deserialize)]
pub enum AmmoTypeTargetType {
    #[serde(rename = "entity")]
    Entity,
    #[serde(rename = "position")]
    Position,
    #[serde(rename = "direction")]
    Direction,
}
fn default_target_type() -> AmmoTypeTargetType {
    AmmoTypeTargetType::Entity
}
