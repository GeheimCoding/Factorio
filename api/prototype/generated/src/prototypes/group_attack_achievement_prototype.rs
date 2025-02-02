#[derive(Debug, serde::Deserialize)]
pub struct GroupAttackAchievementPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::AchievementPrototype,
    #[serde(default = "default_amount")]
    amount: u32,
    entities: Option<Vec<crate::types::EntityID>>,
}
fn default_amount() -> u32 {
    1
}
