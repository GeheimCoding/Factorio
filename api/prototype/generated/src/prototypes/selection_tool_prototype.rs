#[derive(serde::Deserialize)]
pub struct SelectionToolPrototype {
    base_: crate::prototypes::ItemWithLabelPrototype,
    alt_reverse_select: crate::types::SelectionModeData,
    alt_select: crate::types::SelectionModeData,
    always_include_tiles: bool,
    mouse_cursor: crate::types::MouseCursorID,
    reverse_select: crate::types::SelectionModeData,
    select: crate::types::SelectionModeData,
    skip_fog_of_war: bool,
    super_forced_select: crate::types::SelectionModeData,
}
