#[derive(Debug, serde::Deserialize)]
pub struct DontCraftManuallyAchievementPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::AchievementPrototypeWithCondition,
    amount: u32,
}
