#[derive(serde::Deserialize)]
pub struct ResearchAchievementPrototype {
    base_: crate::prototypes::AchievementPrototype,
    research_all: bool,
    technology: crate::types::TechnologyID,
}
