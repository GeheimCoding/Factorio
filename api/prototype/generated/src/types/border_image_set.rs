#[derive(serde::Deserialize)]
pub struct BorderImageSet {
    #[serde(default = "default_border_width")]
    border_width: i32,
    bottom_end: Option<crate::types::Sprite>,
    bottom_left_corner: Option<crate::types::Sprite>,
    bottom_right_corner: Option<crate::types::Sprite>,
    bottom_t: Option<crate::types::Sprite>,
    cross: Option<crate::types::Sprite>,
    horizontal_line: Option<crate::types::Sprite>,
    left_end: Option<crate::types::Sprite>,
    left_t: Option<crate::types::Sprite>,
    right_end: Option<crate::types::Sprite>,
    right_t: Option<crate::types::Sprite>,
    #[serde(default = "default_scale")]
    scale: f64,
    top_end: Option<crate::types::Sprite>,
    top_left_coner: Option<crate::types::Sprite>,
    top_right_corner: Option<crate::types::Sprite>,
    top_t: Option<crate::types::Sprite>,
    vertical_line: Option<crate::types::Sprite>,
}
fn default_border_width() -> i32 {
    0
}
fn default_scale() -> f64 {
    1.0
}
