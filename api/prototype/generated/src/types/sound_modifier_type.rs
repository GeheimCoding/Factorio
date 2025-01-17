#[derive(serde::Deserialize)]
pub enum SoundModifierType {
    #[serde(rename = "game")]
    Game,
    #[serde(rename = "main_menu")]
    MainMenu,
    #[serde(rename = "tips_and_tricks")]
    TipsAndTricks,
    #[serde(rename = "driving")]
    Driving,
    #[serde(rename = "elevation")]
    Elevation,
    #[serde(rename = "space_platform")]
    SpacePlatform,
}
