#[derive(Debug, serde::Deserialize)]
pub struct CurvedRailAPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::RailPrototype,
    // default: `{{-0.7, -2.516}, {0.7, 2.516}}`
    collision_box: Option<crate::types::BoundingBox>,
}
