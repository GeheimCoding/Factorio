#[derive(serde::Deserialize)]
pub struct DontBuildEntityAchievementPrototype {
    base_: crate::prototypes::AchievementPrototypeWithCondition,
    amount: u32,
    dont_build: DontBuildEntityAchievementPrototypeDontBuild,
    research_with: DontBuildEntityAchievementPrototypeResearchWith,
}
#[derive(serde::Deserialize)]
pub enum DontBuildEntityAchievementPrototypeDontBuild {
    #[serde(untagged)]
    EntityID(crate::types::EntityID),
    #[serde(untagged)]
    VecEntityID(Vec<crate::types::EntityID>),
}
#[derive(serde::Deserialize)]
pub enum DontBuildEntityAchievementPrototypeResearchWith {
    #[serde(untagged)]
    ItemID(crate::types::ItemID),
    #[serde(untagged)]
    VecItemID(Vec<crate::types::ItemID>),
}
