#[derive(serde::Deserialize)]
pub struct TreeVariation {
    branch_generation: crate::types::CreateParticleTriggerEffectItem,
    disable_shadow_distortion_beginning_at_frame: Option<u32>,
    leaf_generation: crate::types::CreateParticleTriggerEffectItem,
    leaves: crate::types::Animation,
    normal: Option<crate::types::Animation>,
    overlay: Option<crate::types::Animation>,
    shadow: Option<crate::types::Animation>,
    trunk: crate::types::Animation,
    underwater: Option<crate::types::Animation>,
    #[serde(default = "default_underwater_layer_offset")]
    underwater_layer_offset: i8,
    water_reflection: Option<crate::types::WaterReflectionDefinition>,
}
fn default_underwater_layer_offset() -> i8 {
    1
}
