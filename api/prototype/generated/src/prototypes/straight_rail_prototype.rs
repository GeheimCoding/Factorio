#[derive(serde::Deserialize)]
pub struct StraightRailPrototype {
    base_: crate::prototypes::RailPrototype,
    // default: `{{-0.7, -0.99}, {0.7, 0.99}}`
    collision_box: crate::types::BoundingBox,
}
