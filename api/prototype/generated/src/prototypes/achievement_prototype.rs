#[derive(serde::Deserialize)]
pub struct AchievementPrototype {
    base_: crate::prototypes::Prototype,
    allowed_without_fight: bool,
    icon: crate::types::FileName,
    icon_size: crate::types::SpriteSizeType,
    icons: Vec<crate::types::IconData>,
    steam_stats_name: String,
}
