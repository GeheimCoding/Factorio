pub struct DontBuildEntityAchievementPrototype {
    amount: u32,
    dont_build: DontBuildEntityAchievementPrototypeDontBuild,
    research_with: DontBuildEntityAchievementPrototypeResearchWith,
}
pub enum DontBuildEntityAchievementPrototypeDontBuild {
    EntityID(crate::types::EntityID),
    VecEntityID(Vec<crate::types::EntityID>),
}
pub enum DontBuildEntityAchievementPrototypeResearchWith {
    ItemID(crate::types::ItemID),
    VecItemID(Vec<crate::types::ItemID>),
}
