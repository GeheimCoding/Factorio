#[derive(Debug, serde::Deserialize)]
pub struct WirePosition {
    copper: Option<crate::types::Vector>,
    green: Option<crate::types::Vector>,
    red: Option<crate::types::Vector>,
}
