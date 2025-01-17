#[derive(serde::Deserialize)]
pub struct StraightRailPrototype {
    base_: crate::prototypes::RailPrototype,
    collision_box: crate::types::BoundingBox,
}
