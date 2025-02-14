#[derive(Debug, serde::Deserialize)]
pub enum AmbientSoundType {
    #[serde(rename = "menu-track")]
    MenuTrack,
    #[serde(rename = "main-track")]
    MainTrack,
    #[serde(rename = "hero-track")]
    HeroTrack,
    #[serde(rename = "interlude")]
    Interlude,
}
