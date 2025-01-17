#[derive(serde::Deserialize)]
pub struct ResearchWithSciencePackAchievementPrototype {
    base_: crate::prototypes::AchievementPrototype,
    amount: u32,
    science_pack: crate::types::ItemID,
}
