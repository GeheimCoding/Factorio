pub struct DontResearchBeforeResearchingAchievementPrototype {
    base_: crate::prototypes::AchievementPrototypeWithCondition,
    dont_research: DontResearchBeforeResearchingAchievementPrototypeDontResearch,
    research_with: DontResearchBeforeResearchingAchievementPrototypeResearchWith,
}
#[derive(serde::Deserialize)]
pub enum DontResearchBeforeResearchingAchievementPrototypeDontResearch {
    #[serde(untagged)]
    ItemID(crate::types::ItemID),
    #[serde(untagged)]
    VecItemID(Vec<crate::types::ItemID>),
}
#[derive(serde::Deserialize)]
pub enum DontResearchBeforeResearchingAchievementPrototypeResearchWith {
    #[serde(untagged)]
    ItemID(crate::types::ItemID),
    #[serde(untagged)]
    VecItemID(Vec<crate::types::ItemID>),
}
