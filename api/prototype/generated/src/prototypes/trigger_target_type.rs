#[derive(serde::Deserialize)]
pub struct TriggerTargetType {
    name: String,
    #[serde(rename = "type")]
    type_: String,
}
