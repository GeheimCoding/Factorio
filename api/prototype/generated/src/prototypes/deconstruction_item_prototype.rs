#[derive(Debug, serde::Deserialize)]
pub struct DeconstructionItemPrototype {
    base_: crate::prototypes::SelectionToolPrototype,
    alt_select: crate::types::SelectionModeData,
    #[serde(default = "default_always_include_tiles")]
    always_include_tiles: bool,
    #[serde(default = "default_entity_filter_count")]
    entity_filter_count: crate::types::ItemStackIndex,
    select: crate::types::SelectionModeData,
    stack_size: f64,
    #[serde(default = "default_tile_filter_count")]
    tile_filter_count: crate::types::ItemStackIndex,
}
fn default_always_include_tiles() -> bool {
    false
}
fn default_entity_filter_count() -> crate::types::ItemStackIndex {
    0
}
fn default_tile_filter_count() -> crate::types::ItemStackIndex {
    0
}
