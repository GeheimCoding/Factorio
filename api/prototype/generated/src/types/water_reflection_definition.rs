#[derive(Debug, serde::Deserialize)]
pub struct WaterReflectionDefinition {
    #[serde(default = "default_orientation_to_variation")]
    orientation_to_variation: bool,
    pictures: Option<crate::types::SpriteVariations>,
    #[serde(default = "default_rotate")]
    rotate: bool,
}
fn default_orientation_to_variation() -> bool {
    false
}
fn default_rotate() -> bool {
    false
}
