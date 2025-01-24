#[derive(serde::Deserialize)]
pub struct FusionGeneratorDirectionGraphicsSet {
    animation: Option<crate::types::Animation>,
    fluid_input_graphics: Option<Vec<crate::types::FusionGeneratorFluidInputGraphics>>,
    fusion_effect_uv_map: Option<crate::types::Sprite>,
    working_light: Option<crate::types::Animation>,
}
