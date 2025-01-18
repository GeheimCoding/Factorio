#[derive(serde::Deserialize)]
pub struct CurvedRailAPrototype {
    base_: crate::prototypes::RailPrototype,
    // default: `{{-0.7, -2.516}, {0.7, 2.516}}`
    collision_box: crate::types::BoundingBox,
}
