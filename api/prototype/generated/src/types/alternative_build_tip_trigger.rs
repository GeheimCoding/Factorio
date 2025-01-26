#[derive(Debug, serde::Deserialize)]
pub struct AlternativeBuildTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    #[serde(rename = "type")]
    type_: String,
}
