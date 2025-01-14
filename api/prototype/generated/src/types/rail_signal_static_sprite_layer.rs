pub struct RailSignalStaticSpriteLayer {
    align_to_frame_index: Vec<u8>,
    hide_if_not_connected_to_rails: bool,
    hide_if_simulation: bool,
    render_layer: crate::types::RenderLayer,
    shifts: Vec<crate::types::MapPosition>,
    sprites: crate::types::Animation,
}
