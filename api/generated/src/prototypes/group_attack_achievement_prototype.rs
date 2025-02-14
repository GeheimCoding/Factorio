#[derive(Debug, serde::Deserialize)]
pub struct GroupAttackAchievementPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::AchievementPrototype,
    #[serde(default = "default_amount")]
    amount: u32,
    attack_type: Option<GroupAttackAchievementPrototypeAttackType>,
    entities: Option<crate::vec::Vec<crate::types::EntityID>>,
}
fn default_amount() -> u32 {
    1
}
#[derive(Debug, serde::Deserialize)]
pub enum GroupAttackAchievementPrototypeAttackType {
    #[serde(rename = "autonomous")]
    Autonomous,
    #[serde(rename = "distraction")]
    Distraction,
    #[serde(rename = "scripted")]
    Scripted,
}
