pub struct TileTransitionSpritesheetLayout {
    base_: crate::types::TileSpriteLayoutVariant,
    auxiliary_effect_mask: crate::types::TileTransitionVariantLayout,
    background: crate::types::TileTransitionVariantLayout,
    background_mask: crate::types::TileTransitionVariantLayout,
    double_side_count: u8,
    double_side_line_length: u8,
    double_side_scale: f32,
    double_side_tile_height: u8,
    double_side_x: crate::types::SpriteSizeType,
    double_side_y: crate::types::SpriteSizeType,
    effect_map: crate::types::TileTransitionVariantLayout,
    inner_corner_count: u8,
    inner_corner_line_length: u8,
    inner_corner_scale: f32,
    inner_corner_tile_height: u8,
    inner_corner_x: crate::types::SpriteSizeType,
    inner_corner_y: crate::types::SpriteSizeType,
    lightmap: crate::types::TileTransitionVariantLayout,
    mask: crate::types::TileTransitionVariantLayout,
    o_transition_count: u8,
    o_transition_line_length: u8,
    o_transition_scale: f32,
    o_transition_tile_height: u8,
    o_transition_x: crate::types::SpriteSizeType,
    o_transition_y: crate::types::SpriteSizeType,
    outer_corner_count: u8,
    outer_corner_line_length: u8,
    outer_corner_scale: f32,
    outer_corner_tile_height: u8,
    outer_corner_x: crate::types::SpriteSizeType,
    outer_corner_y: crate::types::SpriteSizeType,
    overlay: crate::types::TileTransitionVariantLayout,
    side_count: u8,
    side_line_length: u8,
    side_scale: f32,
    side_tile_height: u8,
    side_x: crate::types::SpriteSizeType,
    side_y: crate::types::SpriteSizeType,
    u_transition_count: u8,
    u_transition_line_length: u8,
    u_transition_scale: f32,
    u_transition_tile_height: u8,
    u_transition_x: crate::types::SpriteSizeType,
    u_transition_y: crate::types::SpriteSizeType,
}
