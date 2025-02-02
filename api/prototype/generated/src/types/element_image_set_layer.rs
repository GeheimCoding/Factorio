#[derive(Debug, serde::Deserialize)]
pub enum ElementImageSetLayer {
    #[serde(untagged)]
    ElementImageSetLayer {
        #[serde(default = "default_background_blur")]
        background_blur: bool,
        // default: `4` if `background_blur` is `true`
        background_blur_sigma: Option<f32>,
        border: Option<i32>,
        bottom: Option<Box<crate::types::Sprite>>,
        bottom_border: Option<i32>,
        #[serde(default = "default_bottom_outer_border_shift")]
        bottom_outer_border_shift: i32,
        #[serde(default = "default_bottom_tiling")]
        bottom_tiling: bool,
        #[serde(default = "default_bottom_width")]
        bottom_width: crate::types::SpriteSizeType,
        center: Option<Box<crate::types::Sprite>>,
        #[serde(default = "default_center_height")]
        center_height: crate::types::SpriteSizeType,
        #[serde(default = "default_center_tiling_horizontal")]
        center_tiling_horizontal: bool,
        #[serde(default = "default_center_tiling_vertical")]
        center_tiling_vertical: bool,
        #[serde(default = "default_center_width")]
        center_width: crate::types::SpriteSizeType,
        corner_size: Option<ElementImageSetLayerCornerSize>,
        custom_horizontal_tiling_sizes: Option<Vec<u32>>,
        #[serde(default = "default_draw_type")]
        draw_type: ElementImageSetLayerDrawType,
        // default: The `default_tileset` set in GuiStyle
        filename: Option<crate::types::FileName>,
        left: Option<Box<crate::types::Sprite>>,
        left_border: Option<i32>,
        left_bottom: Option<Box<crate::types::Sprite>>,
        #[serde(default = "default_left_height")]
        left_height: crate::types::SpriteSizeType,
        #[serde(default = "default_left_outer_border_shift")]
        left_outer_border_shift: i32,
        #[serde(default = "default_left_tiling")]
        left_tiling: bool,
        left_top: Option<Box<crate::types::Sprite>>,
        #[serde(default = "default_load_in_minimal_mode")]
        load_in_minimal_mode: bool,
        #[serde(default = "default_opacity")]
        opacity: f64,
        #[serde(default = "default_overall_tiling_horizontal_padding")]
        overall_tiling_horizontal_padding: u16,
        #[serde(default = "default_overall_tiling_horizontal_size")]
        overall_tiling_horizontal_size: u16,
        #[serde(default = "default_overall_tiling_horizontal_spacing")]
        overall_tiling_horizontal_spacing: u16,
        #[serde(default = "default_overall_tiling_vertical_padding")]
        overall_tiling_vertical_padding: u16,
        #[serde(default = "default_overall_tiling_vertical_size")]
        overall_tiling_vertical_size: u16,
        #[serde(default = "default_overall_tiling_vertical_spacing")]
        overall_tiling_vertical_spacing: u16,
        position: Option<Box<crate::types::MapPosition>>,
        right: Option<Box<crate::types::Sprite>>,
        right_border: Option<i32>,
        right_bottom: Option<Box<crate::types::Sprite>>,
        #[serde(default = "default_right_height")]
        right_height: crate::types::SpriteSizeType,
        #[serde(default = "default_right_outer_border_shift")]
        right_outer_border_shift: i32,
        #[serde(default = "default_right_tiling")]
        right_tiling: bool,
        right_top: Option<Box<crate::types::Sprite>>,
        #[serde(default = "default_scale")]
        scale: f64,
        #[serde(default = "default_stretch_monolith_image_to_size")]
        stretch_monolith_image_to_size: bool,
        // default: `{r=1, g=1, b=1, a=1}`
        tint: Option<Box<crate::types::Color>>,
        top: Option<Box<crate::types::Sprite>>,
        top_border: Option<i32>,
        #[serde(default = "default_top_outer_border_shift")]
        top_outer_border_shift: i32,
        #[serde(default = "default_top_tiling")]
        top_tiling: bool,
        #[serde(default = "default_top_width")]
        top_width: crate::types::SpriteSizeType,
        // default: `"none"` if this has no other properties, otherwise `"composition"`
        #[serde(rename = "type")]
        type_: Option<ElementImageSetLayerType>,
    },
    #[serde(untagged)]
    Sprite(Box<crate::types::Sprite>),
}
fn default_background_blur() -> bool {
    false
}
fn default_bottom_outer_border_shift() -> i32 {
    0
}
fn default_bottom_tiling() -> bool {
    false
}
fn default_bottom_width() -> crate::types::SpriteSizeType {
    1
}
fn default_center_height() -> crate::types::SpriteSizeType {
    1
}
fn default_center_tiling_horizontal() -> bool {
    false
}
fn default_center_tiling_vertical() -> bool {
    false
}
fn default_center_width() -> crate::types::SpriteSizeType {
    1
}
#[derive(Debug, serde::Deserialize)]
pub enum ElementImageSetLayerCornerSize {
    #[serde(untagged)]
    U16(u16),
    #[serde(untagged)]
    U16U16((u16, u16)),
}
#[derive(Debug, serde::Deserialize)]
pub enum ElementImageSetLayerDrawType {
    #[serde(rename = "inner")]
    Inner,
    #[serde(rename = "outer")]
    Outer,
}
fn default_draw_type() -> ElementImageSetLayerDrawType {
    ElementImageSetLayerDrawType::Inner
}
fn default_left_height() -> crate::types::SpriteSizeType {
    1
}
fn default_left_outer_border_shift() -> i32 {
    0
}
fn default_left_tiling() -> bool {
    false
}
fn default_load_in_minimal_mode() -> bool {
    true
}
fn default_opacity() -> f64 {
    1.0
}
fn default_overall_tiling_horizontal_padding() -> u16 {
    0
}
fn default_overall_tiling_horizontal_size() -> u16 {
    0
}
fn default_overall_tiling_horizontal_spacing() -> u16 {
    0
}
fn default_overall_tiling_vertical_padding() -> u16 {
    0
}
fn default_overall_tiling_vertical_size() -> u16 {
    0
}
fn default_overall_tiling_vertical_spacing() -> u16 {
    0
}
fn default_right_height() -> crate::types::SpriteSizeType {
    1
}
fn default_right_outer_border_shift() -> i32 {
    0
}
fn default_right_tiling() -> bool {
    false
}
fn default_scale() -> f64 {
    1.0
}
fn default_stretch_monolith_image_to_size() -> bool {
    true
}
fn default_top_outer_border_shift() -> i32 {
    0
}
fn default_top_tiling() -> bool {
    false
}
fn default_top_width() -> crate::types::SpriteSizeType {
    1
}
#[derive(Debug, serde::Deserialize)]
pub enum ElementImageSetLayerType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "composition")]
    Composition,
}
