#[derive(Debug, serde::Deserialize)]
pub struct BoolModifier {
    #[serde(flatten)]
    base_: crate::types::BaseModifier,
    modifier: bool,
}
