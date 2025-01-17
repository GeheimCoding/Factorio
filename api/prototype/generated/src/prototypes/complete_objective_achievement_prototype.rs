#[derive(serde::Deserialize)]
pub struct CompleteObjectiveAchievementPrototype {
    base_: crate::prototypes::AchievementPrototypeWithCondition,
    within: crate::types::MapTick,
}
