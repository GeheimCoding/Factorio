#[derive(Debug, serde::Deserialize)]
pub struct SimpleModifier {
    #[serde(flatten)]
    base_: crate::types::BaseModifier,
    modifier: f64,
}
