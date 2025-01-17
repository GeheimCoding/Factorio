#[derive(serde::Deserialize)]
pub struct FlipEntityTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    type_: String,
}
