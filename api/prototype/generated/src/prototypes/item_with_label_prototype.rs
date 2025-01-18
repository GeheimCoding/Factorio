#[derive(serde::Deserialize)]
pub struct ItemWithLabelPrototype {
    base_: crate::prototypes::ItemPrototype,
    // default: Default item text color
    default_label_color: crate::types::Color,
    #[serde(default = "default_draw_label_for_cursor_render")]
    draw_label_for_cursor_render: bool,
}
fn default_draw_label_for_cursor_render() -> bool {
    false
}
