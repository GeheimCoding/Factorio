#[derive(Debug, serde::Deserialize)]
pub struct TileTransitionSpritesheetLayout {
    #[serde(flatten)]
    base_: crate::types::TileSpriteLayoutVariant,
    auxiliary_effect_mask: Option<crate::types::TileTransitionVariantLayout>,
    background: Option<crate::types::TileTransitionVariantLayout>,
    background_mask: Option<crate::types::TileTransitionVariantLayout>,
    double_side_count: Option<u8>,
    double_side_line_length: Option<u8>,
    double_side_scale: Option<f32>,
    double_side_tile_height: Option<u8>,
    double_side_x: Option<crate::types::SpriteSizeType>,
    double_side_y: Option<crate::types::SpriteSizeType>,
    effect_map: Option<crate::types::TileTransitionVariantLayout>,
    inner_corner_count: Option<u8>,
    inner_corner_line_length: Option<u8>,
    inner_corner_scale: Option<f32>,
    inner_corner_tile_height: Option<u8>,
    inner_corner_x: Option<crate::types::SpriteSizeType>,
    inner_corner_y: Option<crate::types::SpriteSizeType>,
    lightmap: Option<crate::types::TileTransitionVariantLayout>,
    mask: Option<crate::types::TileTransitionVariantLayout>,
    o_transition_count: Option<u8>,
    o_transition_line_length: Option<u8>,
    o_transition_scale: Option<f32>,
    o_transition_tile_height: Option<u8>,
    o_transition_x: Option<crate::types::SpriteSizeType>,
    o_transition_y: Option<crate::types::SpriteSizeType>,
    outer_corner_count: Option<u8>,
    outer_corner_line_length: Option<u8>,
    outer_corner_scale: Option<f32>,
    outer_corner_tile_height: Option<u8>,
    outer_corner_x: Option<crate::types::SpriteSizeType>,
    outer_corner_y: Option<crate::types::SpriteSizeType>,
    overlay: Option<crate::types::TileTransitionVariantLayout>,
    side_count: Option<u8>,
    side_line_length: Option<u8>,
    side_scale: Option<f32>,
    side_tile_height: Option<u8>,
    side_x: Option<crate::types::SpriteSizeType>,
    side_y: Option<crate::types::SpriteSizeType>,
    u_transition_count: Option<u8>,
    u_transition_line_length: Option<u8>,
    u_transition_scale: Option<f32>,
    u_transition_tile_height: Option<u8>,
    u_transition_x: Option<crate::types::SpriteSizeType>,
    u_transition_y: Option<crate::types::SpriteSizeType>,
}
