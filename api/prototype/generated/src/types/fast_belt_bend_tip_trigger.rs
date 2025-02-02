#[derive(Debug, serde::Deserialize)]
pub struct FastBeltBendTipTrigger {
    #[serde(flatten)]
    base_: crate::types::CountBasedTipTrigger,
}
