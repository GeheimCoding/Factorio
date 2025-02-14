#[derive(Debug, serde::Deserialize)]
pub struct AchievementPrototypeWithCondition {
    #[serde(flatten)]
    base_: crate::prototypes::AchievementPrototype,
    objective_condition: Option<AchievementPrototypeWithConditionObjectiveCondition>,
}
#[derive(Debug, serde::Deserialize)]
pub enum AchievementPrototypeWithConditionObjectiveCondition {
    #[serde(rename = "game-finished")]
    GameFinished,
    #[serde(rename = "rocket-launched")]
    RocketLaunched,
    #[serde(rename = "late-research")]
    LateResearch,
}
