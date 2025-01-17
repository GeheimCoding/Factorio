#[derive(serde::Deserialize)]
pub struct RotateEntityTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    type_: String,
}
