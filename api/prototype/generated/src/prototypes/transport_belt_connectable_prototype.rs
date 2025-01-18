#[derive(serde::Deserialize)]
pub struct TransportBeltConnectablePrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    #[serde(default = "default_animation_speed_coefficient")]
    animation_speed_coefficient: f64,
    belt_animation_set: crate::types::TransportBeltAnimationSet,
    // default: Empty = `{{0, 0}, {0, 0}}`
    collision_box: crate::types::BoundingBox,
    flags: crate::types::EntityPrototypeFlags,
    speed: f64,
}
fn default_animation_speed_coefficient() -> f64 {
    1.0
}
