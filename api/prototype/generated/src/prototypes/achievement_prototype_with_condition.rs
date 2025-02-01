#[derive(Debug, serde::Deserialize)]
pub struct AchievementPrototypeWithCondition {
    base_: crate::prototypes::AchievementPrototype,
    objective_condition: AchievementPrototypeWithConditionObjectiveCondition,
}
#[derive(Debug, serde::Deserialize)]
pub enum AchievementPrototypeWithConditionObjectiveCondition {
    #[serde(rename = "game-finished")]
    GameFinished,
    #[serde(rename = "rocket-launched")]
    RocketLaunched,
}
