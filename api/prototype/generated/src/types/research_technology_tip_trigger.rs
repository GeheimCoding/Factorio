#[derive(serde::Deserialize)]
pub struct ResearchTechnologyTipTrigger {
    technology: crate::types::TechnologyID,
    type_: String,
}
