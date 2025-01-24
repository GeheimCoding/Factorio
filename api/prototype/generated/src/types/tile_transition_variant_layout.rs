#[derive(serde::Deserialize)]
pub struct TileTransitionVariantLayout {
    base_: crate::types::TileSpriteLayoutVariant,
    double_side: Option<crate::types::TileSpriteLayoutVariant>,
    double_side_count: Option<u8>,
    double_side_line_length: Option<u8>,
    double_side_scale: Option<f32>,
    double_side_tile_height: Option<u8>,
    double_side_x: Option<crate::types::SpriteSizeType>,
    double_side_y: Option<crate::types::SpriteSizeType>,
    inner_corner: Option<crate::types::TileSpriteLayoutVariant>,
    inner_corner_count: Option<u8>,
    inner_corner_line_length: Option<u8>,
    inner_corner_scale: Option<f32>,
    inner_corner_tile_height: Option<u8>,
    inner_corner_x: Option<crate::types::SpriteSizeType>,
    inner_corner_y: Option<crate::types::SpriteSizeType>,
    o_transition: Option<crate::types::TileSpriteLayoutVariant>,
    o_transition_count: Option<u8>,
    o_transition_line_length: Option<u8>,
    o_transition_scale: Option<f32>,
    o_transition_tile_height: Option<u8>,
    o_transition_x: Option<crate::types::SpriteSizeType>,
    o_transition_y: Option<crate::types::SpriteSizeType>,
    outer_corner: Option<crate::types::TileSpriteLayoutVariant>,
    outer_corner_count: Option<u8>,
    outer_corner_line_length: Option<u8>,
    outer_corner_scale: Option<f32>,
    outer_corner_tile_height: Option<u8>,
    outer_corner_x: Option<crate::types::SpriteSizeType>,
    outer_corner_y: Option<crate::types::SpriteSizeType>,
    side: Option<crate::types::TileSpriteLayoutVariant>,
    side_count: Option<u8>,
    side_line_length: Option<u8>,
    side_scale: Option<f32>,
    side_tile_height: Option<u8>,
    side_x: Option<crate::types::SpriteSizeType>,
    side_y: Option<crate::types::SpriteSizeType>,
    u_transition: Option<crate::types::TileSpriteLayoutVariant>,
    u_transition_count: Option<u8>,
    u_transition_line_length: Option<u8>,
    u_transition_scale: Option<f32>,
    u_transition_tile_height: Option<u8>,
    u_transition_x: Option<crate::types::SpriteSizeType>,
    u_transition_y: Option<crate::types::SpriteSizeType>,
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
