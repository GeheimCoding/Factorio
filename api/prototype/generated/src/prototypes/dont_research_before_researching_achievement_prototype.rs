#[derive(Debug, serde::Deserialize)]
pub struct DontResearchBeforeResearchingAchievementPrototype {
    base_: crate::prototypes::AchievementPrototypeWithCondition,
    dont_research: DontResearchBeforeResearchingAchievementPrototypeDontResearch,
    research_with: DontResearchBeforeResearchingAchievementPrototypeResearchWith,
}
#[derive(Debug, serde::Deserialize)]
pub enum DontResearchBeforeResearchingAchievementPrototypeDontResearch {
    #[serde(untagged)]
    ItemID(crate::types::ItemID),
    #[serde(untagged)]
    VecItemID(Vec<crate::types::ItemID>),
}
#[derive(Debug, serde::Deserialize)]
pub enum DontResearchBeforeResearchingAchievementPrototypeResearchWith {
    #[serde(untagged)]
    ItemID(crate::types::ItemID),
    #[serde(untagged)]
    VecItemID(Vec<crate::types::ItemID>),
}
