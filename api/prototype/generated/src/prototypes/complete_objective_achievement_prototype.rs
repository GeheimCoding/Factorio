#[derive(Debug, serde::Deserialize)]
pub struct CompleteObjectiveAchievementPrototype {
    base_: crate::prototypes::AchievementPrototypeWithCondition,
    // default: `math.huge`
    within: Option<crate::types::MapTick>,
}
