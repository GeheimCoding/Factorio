#[derive(serde::Deserialize)]
pub struct BuildEntityAchievementPrototype {
    base_: crate::prototypes::AchievementPrototype,
    #[serde(default = "default_amount")]
    amount: u32,
    #[serde(default = "default_limited_to_one_game")]
    limited_to_one_game: bool,
    to_build: crate::types::EntityID,
    // default: `math.huge`
    within: Option<crate::types::MapTick>,
}
fn default_amount() -> u32 {
    1
}
fn default_limited_to_one_game() -> bool {
    false
}
