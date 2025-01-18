#[derive(serde::Deserialize)]
pub struct ChangeSurfaceTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    surface: String,
    #[serde(rename = "type")]
    type_: String,
}
