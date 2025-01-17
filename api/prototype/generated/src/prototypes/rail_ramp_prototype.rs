#[derive(serde::Deserialize)]
pub struct RailRampPrototype {
    base_: crate::prototypes::RailPrototype,
    collision_box: crate::types::BoundingBox,
    collision_mask_allow_on_deep_oil_ocean: crate::types::CollisionMaskConnector,
    support_range: f32,
}
