#[derive(Debug, serde::Deserialize)]
pub struct CopyPasteToolPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::SelectionToolPrototype,
    alt_select: crate::types::SelectionModeData,
    #[serde(default = "default_always_include_tiles")]
    always_include_tiles: bool,
    #[serde(default = "default_cuts")]
    cuts: bool,
    select: crate::types::SelectionModeData,
    stack_size: f64,
}
fn default_always_include_tiles() -> bool {
    false
}
fn default_cuts() -> bool {
    false
}
