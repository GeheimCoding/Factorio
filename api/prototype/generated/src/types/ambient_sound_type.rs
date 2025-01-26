#[derive(Debug, serde::Deserialize)]
pub enum AmbientSoundType {
    #[serde(rename = "menu_track")]
    MenuTrack,
    #[serde(rename = "main_track")]
    MainTrack,
    #[serde(rename = "hero_track")]
    HeroTrack,
    #[serde(rename = "interlude")]
    Interlude,
}
