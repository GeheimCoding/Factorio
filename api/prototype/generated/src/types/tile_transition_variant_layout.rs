#[derive(serde::Deserialize)]
pub struct TileTransitionVariantLayout {
    base_: crate::types::TileSpriteLayoutVariant,
    double_side: crate::types::TileSpriteLayoutVariant,
    double_side_count: u8,
    double_side_line_length: u8,
    double_side_scale: f32,
    double_side_tile_height: u8,
    double_side_x: crate::types::SpriteSizeType,
    double_side_y: crate::types::SpriteSizeType,
    inner_corner: crate::types::TileSpriteLayoutVariant,
    inner_corner_count: u8,
    inner_corner_line_length: u8,
    inner_corner_scale: f32,
    inner_corner_tile_height: u8,
    inner_corner_x: crate::types::SpriteSizeType,
    inner_corner_y: crate::types::SpriteSizeType,
    o_transition: crate::types::TileSpriteLayoutVariant,
    o_transition_count: u8,
    o_transition_line_length: u8,
    o_transition_scale: f32,
    o_transition_tile_height: u8,
    o_transition_x: crate::types::SpriteSizeType,
    o_transition_y: crate::types::SpriteSizeType,
    outer_corner: crate::types::TileSpriteLayoutVariant,
    outer_corner_count: u8,
    outer_corner_line_length: u8,
    outer_corner_scale: f32,
    outer_corner_tile_height: u8,
    outer_corner_x: crate::types::SpriteSizeType,
    outer_corner_y: crate::types::SpriteSizeType,
    side: crate::types::TileSpriteLayoutVariant,
    side_count: u8,
    side_line_length: u8,
    side_scale: f32,
    side_tile_height: u8,
    side_x: crate::types::SpriteSizeType,
    side_y: crate::types::SpriteSizeType,
    u_transition: crate::types::TileSpriteLayoutVariant,
    u_transition_count: u8,
    u_transition_line_length: u8,
    u_transition_scale: f32,
    u_transition_tile_height: u8,
    u_transition_x: crate::types::SpriteSizeType,
    u_transition_y: crate::types::SpriteSizeType,
    #[serde(default = "default_x_offset")]
    x_offset: crate::types::SpriteSizeType,
    #[serde(default = "default_y_offset")]
    y_offset: crate::types::SpriteSizeType,
}
fn default_x_offset() -> crate::types::SpriteSizeType {
    0
}
fn default_y_offset() -> crate::types::SpriteSizeType {
    0
}
