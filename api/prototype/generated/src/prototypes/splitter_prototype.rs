#[derive(serde::Deserialize)]
pub struct SplitterPrototype {
    base_: crate::prototypes::TransportBeltConnectablePrototype,
    frozen_patch: crate::types::Sprite4Way,
    related_transport_belt: crate::types::EntityID,
    structure: crate::types::Animation4Way,
    #[serde(default = "default_structure_animation_movement_cooldown")]
    structure_animation_movement_cooldown: u32,
    #[serde(default = "default_structure_animation_speed_coefficient")]
    structure_animation_speed_coefficient: f64,
    structure_patch: crate::types::Animation4Way,
}
fn default_structure_animation_movement_cooldown() -> u32 {
    10
}
fn default_structure_animation_speed_coefficient() -> f64 {
    1.0
}
