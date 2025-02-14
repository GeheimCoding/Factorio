#[derive(Debug, serde::Deserialize)]
pub struct LightningRuleBase {
    string: String,
    #[serde(rename = "type")]
    type_: LightningRuleBaseType,
}
#[derive(Debug, serde::Deserialize)]
pub enum LightningRuleBaseType {
    #[serde(rename = "impact-soundset")]
    ImpactSoundset,
    #[serde(rename = "prototype")]
    Prototype,
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "countAsRockForFilteredDeconstruction")]
    CountAsRockForFilteredDeconstruction,
}
