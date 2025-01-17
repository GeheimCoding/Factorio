#[derive(serde::Deserialize)]
pub struct ProgressBarStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    bar: crate::types::ElementImageSet,
    bar_background: crate::types::ElementImageSet,
    bar_width: u32,
    color: crate::types::Color,
    embed_text_in_bar: bool,
    filled_font_color: crate::types::Color,
    font: String,
    font_color: crate::types::Color,
    other_colors: Vec<OtherColors>,
    side_text_padding: i16,
    type_: String,
}
#[derive(serde::Deserialize)]
pub struct OtherColors {
    bar: crate::types::ElementImageSet,
    color: crate::types::Color,
    less_than: f64,
}
