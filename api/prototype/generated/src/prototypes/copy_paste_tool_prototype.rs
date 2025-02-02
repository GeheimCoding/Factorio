#[derive(Debug, serde::Deserialize)]
pub struct CopyPasteToolPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::SelectionToolPrototype,
    #[serde(default = "default_always_include_tiles")]
    always_include_tiles: bool,
    #[serde(default = "default_cuts")]
    cuts: bool,
}
fn default_always_include_tiles() -> bool {
    false
}
fn default_cuts() -> bool {
    false
}
