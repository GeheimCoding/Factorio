#[derive(Debug, serde::Deserialize)]
pub enum ConsumingType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "game-only")]
    GameOnly,
}
