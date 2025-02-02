#[derive(Debug, serde::Deserialize)]
pub struct RailRemnantsPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::CorpsePrototype,
    #[serde(default = "default_build_grid_size")]
    build_grid_size: f64,
    // default: Empty = `{{0, 0}, {0, 0}}`
    collision_box: Option<crate::types::BoundingBox>,
    pictures: crate::types::RailPictureSet,
    related_rail: crate::types::EntityID,
    secondary_collision_box: Option<crate::types::BoundingBox>,
}
fn default_build_grid_size() -> f64 {
    2.0
}
