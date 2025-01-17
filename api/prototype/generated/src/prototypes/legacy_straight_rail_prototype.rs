#[derive(serde::Deserialize)]
pub struct LegacyStraightRailPrototype {
    base_: crate::prototypes::RailPrototype,
    collision_box: crate::types::BoundingBox,
}
