#[derive(serde::Deserialize)]
pub struct BeltTraverseTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    type_: String,
}
