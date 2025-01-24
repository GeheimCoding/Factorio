#[derive(serde::Deserialize)]
pub struct LegacyCurvedRailPrototype {
    base_: crate::prototypes::RailPrototype,
    // default: `{{-0.75, -0.55}, {0.75, 1.6}}`
    collision_box: Option<crate::types::BoundingBox>,
}
