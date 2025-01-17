#[derive(serde::Deserialize)]
pub struct ClearCursorTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    type_: String,
}
