#[derive(Debug, serde::Deserialize)]
pub struct SimpleModifier {
    base_: crate::types::BaseModifier,
    modifier: f64,
}
