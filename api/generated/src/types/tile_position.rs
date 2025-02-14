#[derive(Debug, serde::Deserialize)]
pub enum TilePosition {
    #[serde(untagged)]
    TilePosition { x: i32, y: i32 },
    #[serde(untagged)]
    I32I32((i32, i32)),
}
