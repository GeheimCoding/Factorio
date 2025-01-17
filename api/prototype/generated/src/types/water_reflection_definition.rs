#[derive(serde::Deserialize)]
pub struct WaterReflectionDefinition {
    orientation_to_variation: bool,
    pictures: crate::types::SpriteVariations,
    rotate: bool,
}
