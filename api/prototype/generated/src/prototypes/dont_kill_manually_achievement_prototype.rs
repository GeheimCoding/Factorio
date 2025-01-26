#[derive(Debug, serde::Deserialize)]
pub struct DontKillManuallyAchievementPrototype {
    base_: crate::prototypes::AchievementPrototypeWithCondition,
    to_kill: Option<crate::types::EntityID>,
    type_not_to_kill: Option<String>,
}
