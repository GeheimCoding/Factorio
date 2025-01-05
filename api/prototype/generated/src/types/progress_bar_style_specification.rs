pub struct ProgressBarStyleSpecification {
    bar: crate::types::ElementImageSet,
    bar_background: crate::types::ElementImageSet,
    bar_width: u32,
    color: crate::types::Color,
    embed_text_in_bar: bool,
    filled_font_color: crate::types::Color,
    font: String,
    font_color: crate::types::Color,
    other_colors: Vec<crate::types::OtherColors>,
    side_text_padding: i16,
    type_: String,
}
