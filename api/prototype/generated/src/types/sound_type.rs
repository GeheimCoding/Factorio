#[derive(Debug, serde::Deserialize)]
pub enum SoundType {
    #[serde(rename = "game_effect")]
    GameEffect,
    #[serde(rename = "gui_effect")]
    GuiEffect,
    #[serde(rename = "ambient")]
    Ambient,
    #[serde(rename = "environment")]
    Environment,
    #[serde(rename = "walking")]
    Walking,
    #[serde(rename = "alert")]
    Alert,
    #[serde(rename = "wind")]
    Wind,
    #[serde(rename = "world_ambient")]
    WorldAmbient,
    #[serde(rename = "weapon")]
    Weapon,
    #[serde(rename = "explosion")]
    Explosion,
    #[serde(rename = "enemy")]
    Enemy,
}
