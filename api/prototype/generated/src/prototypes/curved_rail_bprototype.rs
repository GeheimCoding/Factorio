#[derive(Debug, serde::Deserialize)]
pub struct CurvedRailBPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::RailPrototype,
    // default: `{{-0.7, -2.441}, {0.7, 2.441}}`
    collision_box: Option<crate::types::BoundingBox>,
}
