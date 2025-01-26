#[derive(Debug, serde::Deserialize)]
pub struct AchievementPrototypeWithCondition {
    base_: crate::prototypes::AchievementPrototype,
    objective_condition: AchievementPrototypeWithConditionObjectiveCondition,
}
#[derive(Debug, serde::Deserialize)]
pub enum AchievementPrototypeWithConditionObjectiveCondition {
    #[serde(rename = "game_finished")]
    GameFinished,
    #[serde(rename = "rocket_launched")]
    RocketLaunched,
}
