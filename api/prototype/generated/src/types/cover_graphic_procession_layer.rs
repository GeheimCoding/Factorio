#[derive(serde::Deserialize)]
pub struct CoverGraphicProcessionLayer {
    alt_effect: crate::types::CoverGraphicEffectData,
    // default: {0,0}
    distance_traveled_strength: crate::types::Vector,
    effect: crate::types::CoverGraphicEffectData,
    effect_graphic: crate::types::ProcessionGraphic,
    frames: Vec<CoverGraphicProcessionLayerBezierControlPoint>,
    graphic: crate::types::ProcessionGraphic,
    inherit_from: crate::types::ProcessionLayerInheritanceGroupID,
    #[serde(default = "default_is_cloud_effect_advanced")]
    is_cloud_effect_advanced: bool,
    #[serde(default = "default_is_quad_texture")]
    is_quad_texture: bool,
    mask_graphic: crate::types::ProcessionGraphic,
    // default: {1,1}
    pod_movement_strength: crate::types::Vector,
    reference_group: crate::types::ProcessionLayerInheritanceGroupID,
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
    #[serde(default = "default_rotate_with_pod")]
    rotate_with_pod: bool,
    #[serde(default = "default_secondary_draw_order")]
    secondary_draw_order: i8,
    #[serde(default = "default_texture_relative_to")]
    texture_relative_to: crate::types::EffectRelativeTo,
    #[serde(rename = "type")]
    type_: String,
    // default: {512, 512}
    world_size: crate::types::Vector,
}
#[derive(serde::Deserialize)]
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
fn default_is_cloud_effect_advanced() -> bool {
    false
}
fn default_is_quad_texture() -> bool {
    false
}
fn default_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
fn default_rotate_with_pod() -> bool {
    false
}
fn default_secondary_draw_order() -> i8 {
    0
}
fn default_texture_relative_to() -> crate::types::EffectRelativeTo {
    crate::types::EffectRelativeTo::GroundOrigin
}
