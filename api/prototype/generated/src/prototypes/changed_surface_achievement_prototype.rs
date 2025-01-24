#[derive(serde::Deserialize)]
pub struct ChangedSurfaceAchievementPrototype {
    base_: crate::prototypes::AchievementPrototype,
    surface: Option<String>,
}
