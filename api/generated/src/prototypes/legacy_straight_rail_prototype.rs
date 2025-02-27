#[derive(Debug, serde::Deserialize)]
pub struct LegacyStraightRailPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::RailPrototype,
    // default: `{{-0.7, -0.99}, {0.7, 0.99}}`
    collision_box: Option<crate::types::BoundingBox>,
}
