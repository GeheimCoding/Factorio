#[derive(serde::Deserialize)]
pub struct FastBeltBendTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    type_: String,
}
