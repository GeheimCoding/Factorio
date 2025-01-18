#[derive(serde::Deserialize)]
pub enum ElementImageSetLayer {
    #[serde(untagged)]
    ElementImageSetLayer {
        background_blur: bool,
        background_blur_sigma: f32,
        border: i32,
        bottom: crate::types::Sprite,
        bottom_border: i32,
        bottom_outer_border_shift: i32,
        bottom_tiling: bool,
        bottom_width: crate::types::SpriteSizeType,
        center: crate::types::Sprite,
        center_height: crate::types::SpriteSizeType,
        center_tiling_horizontal: bool,
        center_tiling_vertical: bool,
        center_width: crate::types::SpriteSizeType,
        corner_size: ElementImageSetLayerCornerSize,
        custom_horizontal_tiling_sizes: Vec<u32>,
        draw_type: ElementImageSetLayerDrawType,
        filename: crate::types::FileName,
        left: crate::types::Sprite,
        left_border: i32,
        left_bottom: crate::types::Sprite,
        left_height: crate::types::SpriteSizeType,
        left_outer_border_shift: i32,
        left_tiling: bool,
        left_top: crate::types::Sprite,
        load_in_minimal_mode: bool,
        opacity: f64,
        overall_tiling_horizontal_padding: u16,
        overall_tiling_horizontal_size: u16,
        overall_tiling_horizontal_spacing: u16,
        overall_tiling_vertical_padding: u16,
        overall_tiling_vertical_size: u16,
        overall_tiling_vertical_spacing: u16,
        position: crate::types::MapPosition,
        right: crate::types::Sprite,
        right_border: i32,
        right_bottom: crate::types::Sprite,
        right_height: crate::types::SpriteSizeType,
        right_outer_border_shift: i32,
        right_tiling: bool,
        right_top: crate::types::Sprite,
        scale: f64,
        stretch_monolith_image_to_size: bool,
        tint: crate::types::Color,
        top: crate::types::Sprite,
        top_border: i32,
        top_outer_border_shift: i32,
        top_tiling: bool,
        top_width: crate::types::SpriteSizeType,
        #[serde(rename = "type")]
        type_: ElementImageSetLayerType,
    },
    #[serde(untagged)]
    Sprite(Box<crate::types::Sprite>),
}
#[derive(serde::Deserialize)]
pub enum ElementImageSetLayerCornerSize {
    #[serde(untagged)]
    U16(u16),
    #[serde(untagged)]
    U16U16((u16, u16)),
}
#[derive(serde::Deserialize)]
pub enum ElementImageSetLayerDrawType {
    #[serde(rename = "inner")]
    Inner,
    #[serde(rename = "outer")]
    Outer,
}
#[derive(serde::Deserialize)]
pub enum ElementImageSetLayerType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "composition")]
    Composition,
}
