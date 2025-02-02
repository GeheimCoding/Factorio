#[derive(Debug, serde::Deserialize)]
pub struct SignalColorMapping {
    #[serde(flatten)]
    base_: crate::types::SignalIDConnector,
    color: crate::types::Color,
}
