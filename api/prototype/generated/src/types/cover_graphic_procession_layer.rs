pub struct CoverGraphicProcessionLayer {
    alt_effect: crate::types::CoverGraphicEffectData,
    distance_traveled_strength: crate::types::Vector,
    effect: crate::types::CoverGraphicEffectData,
    effect_graphic: crate::types::ProcessionGraphic,
    frames: Vec<CoverGraphicProcessionLayerBezierControlPoint>,
    graphic: crate::types::ProcessionGraphic,
    inherit_from: crate::types::ProcessionLayerInheritanceGroupID,
    is_cloud_effect_advanced: bool,
    is_quad_texture: bool,
    mask_graphic: crate::types::ProcessionGraphic,
    pod_movement_strength: crate::types::Vector,
    reference_group: crate::types::ProcessionLayerInheritanceGroupID,
    render_layer: crate::types::RenderLayer,
    rotate_with_pod: bool,
    secondary_draw_order: i8,
    texture_relative_to: crate::types::EffectRelativeTo,
    type_: String,
    world_size: crate::types::Vector,
}
pub struct CoverGraphicProcessionLayerBezierControlPoint {
    alt_effect_scale_max: f64,
    alt_effect_scale_max_t: f64,
    alt_effect_scale_min: f64,
    alt_effect_scale_min_t: f64,
    alt_effect_shift: crate::types::Vector,
    alt_effect_shift_rate: f64,
    alt_effect_shift_rate_t: f64,
    alt_effect_shift_t: crate::types::Vector,
    effect_scale_max: f64,
    effect_scale_max_t: f64,
    effect_scale_min: f64,
    effect_scale_min_t: f64,
    effect_shift: crate::types::Vector,
    effect_shift_rate: f64,
    effect_shift_rate_t: f64,
    effect_shift_t: crate::types::Vector,
    offset: crate::types::Vector,
    offset_rate: f64,
    offset_rate_t: f64,
    offset_t: crate::types::Vector,
    opacity: f64,
    opacity_t: f64,
    rotation: f64,
    rotation_t: f64,
    timestamp: crate::types::MapTick,
}