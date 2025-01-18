#[derive(serde::Deserialize)]
pub struct RotateEntityTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    #[serde(rename = "type")]
    type_: String,
}
