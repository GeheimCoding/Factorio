#[derive(Debug, serde::Deserialize)]
pub struct ResearchWithSciencePackAchievementPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::AchievementPrototype,
    #[serde(default = "default_amount")]
    amount: u32,
    science_pack: crate::types::ItemID,
}
fn default_amount() -> u32 {
    1
}
