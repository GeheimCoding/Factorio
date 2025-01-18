#[derive(serde::Deserialize)]
pub struct FogMaskShapeDefinition {
    #[serde(default = "default_falloff")]
    falloff: f32,
    rect: crate::types::SimpleBoundingBox,
}
fn default_falloff() -> f32 {
    1.0
}
