#[derive(Debug, serde::Deserialize)]
pub struct ChangedSurfaceAchievementPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::AchievementPrototype,
    surface: Option<String>,
}
