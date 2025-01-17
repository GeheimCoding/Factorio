#[derive(serde::Deserialize)]
pub struct SignalColorMapping {
    base_: crate::types::SignalIDConnector,
    color: crate::types::Color,
}
