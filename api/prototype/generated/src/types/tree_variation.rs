pub struct TreeVariation {
    branch_generation: crate::types::CreateParticleTriggerEffectItem,
    disable_shadow_distortion_beginning_at_frame: u32,
    leaf_generation: crate::types::CreateParticleTriggerEffectItem,
    leaves: crate::types::Animation,
    normal: crate::types::Animation,
    overlay: crate::types::Animation,
    shadow: crate::types::Animation,
    trunk: crate::types::Animation,
    underwater: crate::types::Animation,
    underwater_layer_offset: i8,
    water_reflection: crate::types::WaterReflectionDefinition,
}
