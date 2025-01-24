#[derive(serde::Deserialize)]
pub struct ResearchAchievementPrototype {
    base_: crate::prototypes::AchievementPrototype,
    #[serde(default = "default_research_all")]
    research_all: bool,
    technology: Option<crate::types::TechnologyID>,
}
fn default_research_all() -> bool {
    false
}
