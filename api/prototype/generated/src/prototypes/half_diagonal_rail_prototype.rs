#[derive(Debug, serde::Deserialize)]
pub struct HalfDiagonalRailPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::RailPrototype,
    // default: `{{-0.75, -1.9}, {0.75, 1.9}}`
    collision_box: Option<crate::types::BoundingBox>,
}
