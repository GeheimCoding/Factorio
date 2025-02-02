#[derive(Debug, serde::Deserialize)]
pub struct ClearCursorTipTrigger {
    #[serde(flatten)]
    base_: crate::types::CountBasedTipTrigger,
}
