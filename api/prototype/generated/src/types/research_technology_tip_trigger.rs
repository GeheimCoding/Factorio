#[derive(serde::Deserialize)]
pub struct ResearchTechnologyTipTrigger {
    technology: crate::types::TechnologyID,
    #[serde(rename = "type")]
    type_: String,
}
