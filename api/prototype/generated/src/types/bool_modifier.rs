#[derive(Debug, serde::Deserialize)]
pub struct BoolModifier {
    base_: crate::types::BaseModifier,
    modifier: bool,
}
