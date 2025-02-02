#[derive(Debug, serde::Deserialize)]
pub struct ChangeSurfaceTipTrigger {
    #[serde(flatten)]
    base_: crate::types::CountBasedTipTrigger,
    surface: String,
}
