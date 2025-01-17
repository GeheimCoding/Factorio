#[derive(serde::Deserialize)]
pub struct WireConnectionPoint {
    shadow: crate::types::WirePosition,
    wire: crate::types::WirePosition,
}
