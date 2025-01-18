#[derive(serde::Deserialize)]
pub struct ResearchWithSciencePackTipTrigger {
    science_pack: crate::types::ItemID,
    #[serde(rename = "type")]
    type_: String,
}
