#[derive(serde::Deserialize)]
pub struct CreatePlatformAchievementPrototype {
    base_: crate::prototypes::AchievementPrototype,
    amount: u32,
}
