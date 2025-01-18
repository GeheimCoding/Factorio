#[derive(serde::Deserialize)]
pub struct BorderImageSet {
    #[serde(default = "default_border_width")]
    border_width: i32,
    bottom_end: crate::types::Sprite,
    bottom_left_corner: crate::types::Sprite,
    bottom_right_corner: crate::types::Sprite,
    bottom_t: crate::types::Sprite,
    cross: crate::types::Sprite,
    horizontal_line: crate::types::Sprite,
    left_end: crate::types::Sprite,
    left_t: crate::types::Sprite,
    right_end: crate::types::Sprite,
    right_t: crate::types::Sprite,
    #[serde(default = "default_scale")]
    scale: f64,
    top_end: crate::types::Sprite,
    top_left_coner: crate::types::Sprite,
    top_right_corner: crate::types::Sprite,
    top_t: crate::types::Sprite,
    vertical_line: crate::types::Sprite,
}
fn default_border_width() -> i32 {
    0
}
fn default_scale() -> f64 {
    1.0
}
