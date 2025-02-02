#[derive(Debug, serde::Deserialize)]
pub struct CompleteObjectiveAchievementPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::AchievementPrototypeWithCondition,
    // default: `math.huge`
    within: Option<crate::types::MapTick>,
}
