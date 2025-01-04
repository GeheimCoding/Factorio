pub struct TreeVariation {
    branch_generation: CreateParticleTriggerEffectItem,
    disable_shadow_distortion_beginning_at_frame: u32,
    leaf_generation: CreateParticleTriggerEffectItem,
    leaves: Animation,
    normal: Animation,
    overlay: Animation,
    shadow: Animation,
    trunk: Animation,
    underwater: Animation,
    underwater_layer_offset: i8,
    water_reflection: WaterReflectionDefinition,
}
