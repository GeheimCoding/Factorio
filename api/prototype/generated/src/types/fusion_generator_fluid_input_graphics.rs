#[derive(Debug, serde::Deserialize)]
pub struct FusionGeneratorFluidInputGraphics {
    fusion_effect_uv_map: Option<crate::types::Sprite>,
    sprite: Option<crate::types::Sprite>,
    working_light: Option<crate::types::Sprite>,
}
