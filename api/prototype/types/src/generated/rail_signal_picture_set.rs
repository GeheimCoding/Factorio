pub struct RailSignalPictureSet {
    circuit_connector: Vec<CircuitConnectorDefinition>,
    circuit_connector_render_layer: RenderLayer,
    lights: RailSignalLights,
    rail_piece: RailSignalStaticSpriteLayer,
    selection_box_shift: Vec<Vector>,
    signal_color_to_structure_frame_index: RailSignalColorToFrameIndex,
    structure: RotatedAnimation,
    structure_align_to_animation_index: Vec<u8>,
    structure_render_layer: RenderLayer,
    upper_rail_piece: RailSignalStaticSpriteLayer,
}
