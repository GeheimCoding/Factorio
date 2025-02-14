#[derive(Debug, serde::Deserialize)]
pub struct RailSignalPictureSet {
    circuit_connector: Option<crate::vec::Vec<crate::types::CircuitConnectorDefinition>>,
    #[serde(default = "default_circuit_connector_render_layer")]
    circuit_connector_render_layer: crate::types::RenderLayer,
    lights: crate::types::RailSignalLights,
    rail_piece: Option<crate::types::RailSignalStaticSpriteLayer>,
    selection_box_shift: Option<crate::vec::Vec<crate::types::Vector>>,
    signal_color_to_structure_frame_index: crate::types::RailSignalColorToFrameIndex,
    structure: crate::types::RotatedAnimation,
    structure_align_to_animation_index: Option<crate::vec::Vec<u8>>,
    #[serde(default = "default_structure_render_layer")]
    structure_render_layer: crate::types::RenderLayer,
    upper_rail_piece: Option<crate::types::RailSignalStaticSpriteLayer>,
}
fn default_circuit_connector_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
fn default_structure_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::FloorMechanics
}
