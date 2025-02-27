#[derive(Debug, serde::Deserialize)]
pub struct ProgressBarStyleSpecification {
    #[serde(flatten)]
    base_: crate::types::BaseStyleSpecification,
    bar: Option<crate::types::ElementImageSet>,
    bar_background: Option<crate::types::ElementImageSet>,
    bar_width: Option<u32>,
    color: Option<crate::types::Color>,
    embed_text_in_bar: Option<bool>,
    filled_font_color: Option<crate::types::Color>,
    font: Option<String>,
    font_color: Option<crate::types::Color>,
    other_colors: Option<crate::vec::Vec<OtherColors>>,
    side_text_padding: Option<i16>,
}
#[derive(Debug, serde::Deserialize)]
pub struct OtherColors {
    bar: Option<crate::types::ElementImageSet>,
    color: Option<crate::types::Color>,
    less_than: f64,
}
