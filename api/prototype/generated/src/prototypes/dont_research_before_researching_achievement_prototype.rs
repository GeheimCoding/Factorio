pub struct DontResearchBeforeResearchingAchievementPrototype {
    base_: crate::prototypes::AchievementPrototypeWithCondition,
    dont_research: DontResearchBeforeResearchingAchievementPrototypeDontResearch,
    research_with: DontResearchBeforeResearchingAchievementPrototypeResearchWith,
}
pub enum DontResearchBeforeResearchingAchievementPrototypeDontResearch {
    ItemID(crate::types::ItemID),
    VecItemID(Vec<crate::types::ItemID>),
}
pub enum DontResearchBeforeResearchingAchievementPrototypeResearchWith {
    ItemID(crate::types::ItemID),
    VecItemID(Vec<crate::types::ItemID>),
}
