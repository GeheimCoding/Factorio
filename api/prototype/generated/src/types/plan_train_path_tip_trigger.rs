#[derive(Debug, serde::Deserialize)]
pub struct PlanTrainPathTipTrigger {
    distance: f64,
    #[serde(rename = "type")]
    type_: String,
}
