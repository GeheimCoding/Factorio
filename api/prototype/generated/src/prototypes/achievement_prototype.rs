#[derive(serde::Deserialize)]
pub struct AchievementPrototype {
    base_: crate::prototypes::Prototype,
    #[serde(default = "default_allowed_without_fight")]
    allowed_without_fight: bool,
    icon: crate::types::FileName,
    #[serde(default = "default_icon_size")]
    icon_size: crate::types::SpriteSizeType,
    icons: Vec<crate::types::IconData>,
    #[serde(default = "default_steam_stats_name")]
    steam_stats_name: String,
}
fn default_allowed_without_fight() -> bool {
    true
}
fn default_icon_size() -> crate::types::SpriteSizeType {
    64
}
fn default_steam_stats_name() -> String {
    String::from("")
}
