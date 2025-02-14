#[derive(Debug, serde::Deserialize)]
pub struct SpacePlatformTileDefinition {
    position: crate::types::TilePosition,
    tile: crate::types::TileID,
}
