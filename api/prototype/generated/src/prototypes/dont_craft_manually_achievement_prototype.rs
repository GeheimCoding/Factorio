#[derive(Debug, serde::Deserialize)]
pub struct DontCraftManuallyAchievementPrototype {
    base_: crate::prototypes::AchievementPrototypeWithCondition,
    amount: u32,
}
