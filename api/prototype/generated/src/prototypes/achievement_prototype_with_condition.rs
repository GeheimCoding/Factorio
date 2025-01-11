pub struct AchievementPrototypeWithCondition {
    base_: crate::prototypes::AchievementPrototype,
    objective_condition: AchievementPrototypeWithConditionObjectiveCondition,
}
pub enum AchievementPrototypeWithConditionObjectiveCondition {
    GameFinished,
    RocketLaunched,
}
