#[derive(Debug, serde::Deserialize)]
pub struct TransportBeltConnectablePrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityWithOwnerPrototype,
    #[serde(default = "default_animation_speed_coefficient")]
    animation_speed_coefficient: f64,
    // overridden by some child
    belt_animation_set: Option<crate::types::TransportBeltAnimationSet>,
    // default: Empty = `{{0, 0}, {0, 0}}`
    collision_box: Option<crate::types::BoundingBox>,
    flags: Option<crate::types::EntityPrototypeFlags>,
    speed: f64,
}
fn default_animation_speed_coefficient() -> f64 {
    1.0
}
