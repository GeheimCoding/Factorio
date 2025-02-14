#[derive(Debug, serde::Deserialize)]
pub struct DontBuildEntityAchievementPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::AchievementPrototypeWithCondition,
    #[serde(default = "default_amount")]
    amount: u32,
    dont_build: DontBuildEntityAchievementPrototypeDontBuild,
    research_with: Option<DontBuildEntityAchievementPrototypeResearchWith>,
}
fn default_amount() -> u32 {
    0
}
#[derive(Debug, serde::Deserialize)]
pub enum DontBuildEntityAchievementPrototypeDontBuild {
    #[serde(untagged)]
    EntityID(crate::types::EntityID),
    #[serde(untagged)]
    VecEntityID(crate::vec::Vec<crate::types::EntityID>),
}
#[derive(Debug, serde::Deserialize)]
pub enum DontBuildEntityAchievementPrototypeResearchWith {
    #[serde(untagged)]
    ItemID(crate::types::ItemID),
    #[serde(untagged)]
    VecItemID(crate::vec::Vec<crate::types::ItemID>),
}
