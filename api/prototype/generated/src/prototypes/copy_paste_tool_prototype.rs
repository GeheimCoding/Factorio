#[derive(serde::Deserialize)]
pub struct CopyPasteToolPrototype {
    base_: crate::prototypes::SelectionToolPrototype,
    alt_select: crate::types::SelectionModeData,
    always_include_tiles: bool,
    cuts: bool,
    select: crate::types::SelectionModeData,
    stack_size: String,
}
