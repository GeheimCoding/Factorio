pub struct SingleGraphicProcessionLayer {
    animation_driven_by_curve: bool,
    clip_with_hatches: bool,
    compensated_pivot: bool,
    frames: Vec<crate::types::SingleGraphicLayerProcessionBezierControlPoint>,
    graphic: crate::types::ProcessionGraphic,
    is_passenger_only: bool,
    relative_to: crate::types::EffectRelativeTo,
    render_layer: crate::types::RenderLayer,
    rotates_with_pod: bool,
    secondary_draw_order: i8,
    shift_rotates_with_pod: bool,
    type_: SingleGraphic,
}
