pub struct AmmoType {
    action: crate::types::Trigger,
    clamp_position: bool,
    consumption_modifier: f32,
    cooldown_modifier: f64,
    energy_consumption: crate::types::Energy,
    range_modifier: f64,
    source_type: crate::types::AmmoSourceType,
    target_filter: Vec<crate::types::EntityID>,
    target_type: AmmoTypeTargetType,
}
#[derive(serde::Deserialize)]
pub enum AmmoTypeTargetType {
    #[serde(rename = "entity")]
    Entity,
    #[serde(rename = "position")]
    Position,
    #[serde(rename = "direction")]
    Direction,
}
