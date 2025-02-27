#[derive(Debug, serde::Deserialize)]
pub struct SplitterPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::TransportBeltConnectablePrototype,
    frozen_patch: Option<crate::types::Sprite4Way>,
    related_transport_belt: Option<crate::types::EntityID>,
    structure: Option<crate::types::Animation4Way>,
    #[serde(default = "default_structure_animation_movement_cooldown")]
    structure_animation_movement_cooldown: u32,
    #[serde(default = "default_structure_animation_speed_coefficient")]
    structure_animation_speed_coefficient: f64,
    structure_patch: Option<crate::types::Animation4Way>,
}
fn default_structure_animation_movement_cooldown() -> u32 {
    10
}
fn default_structure_animation_speed_coefficient() -> f64 {
    1.0
}
