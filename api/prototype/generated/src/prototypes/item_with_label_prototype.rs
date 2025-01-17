#[derive(serde::Deserialize)]
pub struct ItemWithLabelPrototype {
    base_: crate::prototypes::ItemPrototype,
    default_label_color: crate::types::Color,
    draw_label_for_cursor_render: bool,
}
