#[derive(Debug, serde::Deserialize)]
pub enum TipStatus {
    #[serde(rename = "locked")]
    Locked,
    #[serde(rename = "optional")]
    Optional,
    #[serde(rename = "dependencies-not-met")]
    DependenciesNotMet,
    #[serde(rename = "unlocked")]
    Unlocked,
    #[serde(rename = "suggested")]
    Suggested,
    #[serde(rename = "not-to-be-suggested")]
    NotToBeSuggested,
    #[serde(rename = "completed-without-tutorial")]
    CompletedWithoutTutorial,
    #[serde(rename = "completed")]
    Completed,
}
