pub struct ProgressBarStyleSpecification {
    bar: ElementImageSet,
    bar_background: ElementImageSet,
    bar_width: u32,
    color: Color,
    embed_text_in_bar: bool,
    filled_font_color: Color,
    font: String,
    font_color: Color,
    other_colors: Vec<OtherColors>,
    side_text_padding: i16,
    type_: ProgressbarStyle,
}
