#[derive(serde::Deserialize)]
pub struct CurvedRailBPrototype {
    base_: crate::prototypes::RailPrototype,
    collision_box: crate::types::BoundingBox,
}
