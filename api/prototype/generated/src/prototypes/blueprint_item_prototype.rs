#[derive(Debug, serde::Deserialize)]
pub struct BlueprintItemPrototype {
    base_: crate::prototypes::SelectionToolPrototype,
    alt_select: crate::types::SelectionModeData,
    #[serde(default = "default_always_include_tiles")]
    always_include_tiles: bool,
    #[serde(default = "default_draw_label_for_cursor_render")]
    draw_label_for_cursor_render: bool,
    select: crate::types::SelectionModeData,
    stack_size: f64,
}
fn default_always_include_tiles() -> bool {
    false
}
fn default_draw_label_for_cursor_render() -> bool {
    true
}
