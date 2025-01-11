pub struct DontKillManuallyAchievementPrototype {
    base_: crate::prototypes::AchievementPrototypeWithCondition,
    to_kill: crate::types::EntityID,
    type_not_to_kill: String,
}
