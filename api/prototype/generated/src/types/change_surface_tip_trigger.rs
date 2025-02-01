#[derive(Debug, serde::Deserialize)]
pub struct ChangeSurfaceTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    surface: String,
}
