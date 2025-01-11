pub struct BuildEntityAchievementPrototype {
    amount: u32,
    limited_to_one_game: bool,
    to_build: crate::types::EntityID,
    within: crate::types::MapTick,
}
