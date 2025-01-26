#[derive(Debug, serde::Deserialize)]
pub struct CreatePlatformAchievementPrototype {
    base_: crate::prototypes::AchievementPrototype,
    #[serde(default = "default_amount")]
    amount: u32,
}
fn default_amount() -> u32 {
    1
}
