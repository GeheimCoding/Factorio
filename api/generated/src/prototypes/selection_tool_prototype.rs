#[derive(Debug, serde::Deserialize)]
pub struct SelectionToolPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::ItemWithLabelPrototype,
    alt_reverse_select: Option<crate::types::SelectionModeData>,
    // overridden by some child
    alt_select: Option<crate::types::SelectionModeData>,
    // overridden by some child
    #[serde(default = "default_always_include_tiles")]
    always_include_tiles: bool,
    #[serde(default = "default_mouse_cursor")]
    mouse_cursor: crate::types::MouseCursorID,
    reverse_select: Option<crate::types::SelectionModeData>,
    // overridden by some child
    select: Option<crate::types::SelectionModeData>,
    #[serde(default = "default_skip_fog_of_war")]
    skip_fog_of_war: bool,
    super_forced_select: Option<crate::types::SelectionModeData>,
}
fn default_always_include_tiles() -> bool {
    false
}
fn default_mouse_cursor() -> crate::types::MouseCursorID {
    String::from("selection-tool-cursor")
}
fn default_skip_fog_of_war() -> bool {
    false
}
