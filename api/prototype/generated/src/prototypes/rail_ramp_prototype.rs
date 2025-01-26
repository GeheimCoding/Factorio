#[derive(Debug, serde::Deserialize)]
pub struct RailRampPrototype {
    base_: crate::prototypes::RailPrototype,
    // default: `{{-1.6, -7.6}, {1.6, 7.6}}`
    collision_box: Option<crate::types::BoundingBox>,
    collision_mask_allow_on_deep_oil_ocean: Option<crate::types::CollisionMaskConnector>,
    #[serde(default = "default_support_range")]
    support_range: f32,
}
fn default_support_range() -> f32 {
    15.0
}
