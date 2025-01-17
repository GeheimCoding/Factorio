#[derive(serde::Deserialize)]
pub struct DepleteResourceAchievementPrototype {
    base_: crate::prototypes::AchievementPrototype,
    amount: u32,
    limited_to_one_game: bool,
}
