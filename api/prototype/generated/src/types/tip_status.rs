#[derive(Debug, serde::Deserialize)]
pub enum TipStatus {
    #[serde(rename = "locked")]
    Locked,
    #[serde(rename = "optional")]
    Optional,
    #[serde(rename = "dependencies_not_met")]
    DependenciesNotMet,
    #[serde(rename = "unlocked")]
    Unlocked,
    #[serde(rename = "suggested")]
    Suggested,
    #[serde(rename = "not_to_be_suggested")]
    NotToBeSuggested,
    #[serde(rename = "completed_without_tutorial")]
    CompletedWithoutTutorial,
    #[serde(rename = "completed")]
    Completed,
}
