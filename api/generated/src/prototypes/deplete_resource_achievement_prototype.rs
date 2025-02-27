#[derive(Debug, serde::Deserialize)]
pub struct DepleteResourceAchievementPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::AchievementPrototype,
    #[serde(default = "default_amount")]
    amount: u32,
    #[serde(default = "default_limited_to_one_game")]
    limited_to_one_game: bool,
}
fn default_amount() -> u32 {
    1
}
fn default_limited_to_one_game() -> bool {
    false
}
