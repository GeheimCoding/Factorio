#[derive(serde::Deserialize)]
pub struct FogMaskShapeDefinition {
    falloff: f32,
    rect: crate::types::SimpleBoundingBox,
}
