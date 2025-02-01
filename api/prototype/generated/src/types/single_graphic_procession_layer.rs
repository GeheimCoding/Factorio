#[derive(Debug, serde::Deserialize)]
pub struct SingleGraphicProcessionLayer {
    #[serde(default = "default_animation_driven_by_curve")]
    animation_driven_by_curve: bool,
    #[serde(default = "default_clip_with_hatches")]
    clip_with_hatches: bool,
    #[serde(default = "default_compensated_pivot")]
    compensated_pivot: bool,
    frames: Vec<SingleGraphicLayerProcessionBezierControlPoint>,
    graphic: crate::types::ProcessionGraphic,
    #[serde(default = "default_is_passenger_only")]
    is_passenger_only: bool,
    #[serde(default = "default_relative_to")]
    relative_to: crate::types::EffectRelativeTo,
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
    #[serde(default = "default_rotates_with_pod")]
    rotates_with_pod: bool,
    #[serde(default = "default_secondary_draw_order")]
    secondary_draw_order: i8,
    #[serde(default = "default_shift_rotates_with_pod")]
    shift_rotates_with_pod: bool,
}
fn default_animation_driven_by_curve() -> bool {
    false
}
fn default_clip_with_hatches() -> bool {
    true
}
fn default_compensated_pivot() -> bool {
    true
}
#[derive(Debug, serde::Deserialize)]
pub struct SingleGraphicLayerProcessionBezierControlPoint {
    frame: f32,
    opacity: Option<f64>,
    opacity_t: Option<f64>,
    rotation: Option<f64>,
    rotation_t: Option<f64>,
    scale: Option<f64>,
    scale_t: Option<f64>,
    shift: Option<crate::types::Vector>,
    shift_rate: Option<f64>,
    shift_rate_t: Option<f64>,
    shift_t: Option<crate::types::Vector>,
    timestamp: Option<crate::types::MapTick>,
    tint: Option<crate::types::Color>,
    tint_t: Option<crate::types::Color>,
}
fn default_is_passenger_only() -> bool {
    false
}
fn default_relative_to() -> crate::types::EffectRelativeTo {
    crate::types::EffectRelativeTo::Pod
}
fn default_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
fn default_rotates_with_pod() -> bool {
    true
}
fn default_secondary_draw_order() -> i8 {
    0
}
fn default_shift_rotates_with_pod() -> bool {
    true
}
