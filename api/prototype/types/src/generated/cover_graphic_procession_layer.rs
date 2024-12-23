pub struct CoverGraphicProcessionLayer {
    alt_effect: CoverGraphicEffectData,
    distance_traveled_strength: Vector,
    effect: CoverGraphicEffectData,
    effect_graphic: ProcessionGraphic,
    frames: Vec<CoverGraphicProcessionLayerBezierControlPoint>,
    graphic: ProcessionGraphic,
    inherit_from: ProcessionLayerInheritanceGroupID,
    is_cloud_effect_advanced: bool,
    is_quad_texture: bool,
    mask_graphic: ProcessionGraphic,
    pod_movement_strength: Vector,
    reference_group: ProcessionLayerInheritanceGroupID,
    render_layer: RenderLayer,
    rotate_with_pod: bool,
    secondary_draw_order: i8,
    texture_relative_to: EffectRelativeTo,
    type_: CoverGraphic,
    world_size: Vector,
}
