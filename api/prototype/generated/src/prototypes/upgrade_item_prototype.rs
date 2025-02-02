#[derive(Debug, serde::Deserialize)]
pub struct UpgradeItemPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::SelectionToolPrototype,
    #[serde(default = "default_always_include_tiles")]
    always_include_tiles: bool,
    #[serde(default = "default_draw_label_for_cursor_render")]
    draw_label_for_cursor_render: bool,
}
fn default_always_include_tiles() -> bool {
    false
}
fn default_draw_label_for_cursor_render() -> bool {
    true
}
