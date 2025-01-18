#[derive(serde::Deserialize)]
pub struct MaxSuccessfulAttemptsPerTickPerConstructionQueueModifier {
    base_: crate::types::SimpleModifier,
    #[serde(rename = "type")]
    type_: String,
    use_icon_overlay_constant: bool,
}
