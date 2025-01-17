#[derive(serde::Deserialize)]
pub struct CurvedRailAPrototype {
    base_: crate::prototypes::RailPrototype,
    collision_box: crate::types::BoundingBox,
}
