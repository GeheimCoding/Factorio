#[derive(serde::Deserialize)]
pub struct CompleteObjectiveAchievementPrototype {
    base_: crate::prototypes::AchievementPrototypeWithCondition,
    // default: `math.huge`
    within: crate::types::MapTick,
}
