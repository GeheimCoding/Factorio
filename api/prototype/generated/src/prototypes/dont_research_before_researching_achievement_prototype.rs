#[derive(Debug, serde::Deserialize)]
pub struct DontResearchBeforeResearchingAchievementPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::AchievementPrototypeWithCondition,
    dont_research: DontResearchBeforeResearchingAchievementPrototypeDontResearch,
    research_with: DontResearchBeforeResearchingAchievementPrototypeResearchWith,
}
#[derive(Debug, serde::Deserialize)]
pub enum DontResearchBeforeResearchingAchievementPrototypeDontResearch {
    #[serde(untagged)]
    ItemID(crate::types::ItemID),
    #[serde(untagged)]
    VecItemID(crate::vec::Vec<crate::types::ItemID>),
}
#[derive(Debug, serde::Deserialize)]
pub enum DontResearchBeforeResearchingAchievementPrototypeResearchWith {
    #[serde(untagged)]
    ItemID(crate::types::ItemID),
    #[serde(untagged)]
    VecItemID(crate::vec::Vec<crate::types::ItemID>),
}
