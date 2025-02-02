#[derive(Debug, serde::Deserialize)]
pub struct TileTransitionsToTiles {
    #[serde(flatten)]
    base_: crate::types::TileTransitions,
    to_tiles: crate::vec::Vec<crate::types::TileID>,
    transition_group: u8,
}
