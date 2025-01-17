#[derive(serde::Deserialize)]
pub enum ConsumingType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "game_only")]
    GameOnly,
}
