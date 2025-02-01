#[derive(Debug, serde::Deserialize)]
pub enum SoundModifierType {
    #[serde(rename = "game")]
    Game,
    #[serde(rename = "main-menu")]
    MainMenu,
    #[serde(rename = "tips-and-tricks")]
    TipsAndTricks,
    #[serde(rename = "driving")]
    Driving,
    #[serde(rename = "elevation")]
    Elevation,
    #[serde(rename = "space-platform")]
    SpacePlatform,
}
