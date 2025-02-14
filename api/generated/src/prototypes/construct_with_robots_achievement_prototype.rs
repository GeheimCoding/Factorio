#[derive(Debug, serde::Deserialize)]
pub struct ConstructWithRobotsAchievementPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::AchievementPrototype,
    #[serde(default = "default_amount")]
    amount: u32,
    limited_to_one_game: bool,
    #[serde(default = "default_more_than_manually")]
    more_than_manually: bool,
}
fn default_amount() -> u32 {
    0
}
fn default_more_than_manually() -> bool {
    false
}
