#[derive(serde::Deserialize)]
pub struct SingleGraphicProcessionLayer {
    animation_driven_by_curve: bool,
    clip_with_hatches: bool,
    compensated_pivot: bool,
    frames: Vec<SingleGraphicLayerProcessionBezierControlPoint>,
    graphic: crate::types::ProcessionGraphic,
    is_passenger_only: bool,
    relative_to: crate::types::EffectRelativeTo,
    render_layer: crate::types::RenderLayer,
    rotates_with_pod: bool,
    secondary_draw_order: i8,
    shift_rotates_with_pod: bool,
    #[serde(rename = "type")]
    type_: String,
}
#[derive(serde::Deserialize)]
pub struct SingleGraphicLayerProcessionBezierControlPoint {
    frame: f32,
    opacity: f64,
    opacity_t: f64,
    rotation: f64,
    rotation_t: f64,
    scale: f64,
    scale_t: f64,
    shift: crate::types::Vector,
    shift_rate: f64,
    shift_rate_t: f64,
    shift_t: crate::types::Vector,
    timestamp: crate::types::MapTick,
    tint: crate::types::Color,
    tint_t: crate::types::Color,
}
