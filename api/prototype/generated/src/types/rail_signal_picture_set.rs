pub struct RailSignalPictureSet {
    circuit_connector: Vec<crate::types::CircuitConnectorDefinition>,
    circuit_connector_render_layer: crate::types::RenderLayer,
    lights: crate::types::RailSignalLights,
    rail_piece: crate::types::RailSignalStaticSpriteLayer,
    selection_box_shift: Vec<crate::types::Vector>,
    signal_color_to_structure_frame_index: crate::types::RailSignalColorToFrameIndex,
    structure: crate::types::RotatedAnimation,
    structure_align_to_animation_index: Vec<u8>,
    structure_render_layer: crate::types::RenderLayer,
    upper_rail_piece: crate::types::RailSignalStaticSpriteLayer,
}
