#[derive(serde::Deserialize)]
pub struct LightningRuleBase {
    string: String,
    #[serde(rename = "type")]
    type_: LightningRuleBaseType,
}
#[derive(serde::Deserialize)]
pub enum LightningRuleBaseType {
    #[serde(rename = "impact_soundset")]
    ImpactSoundset,
    #[serde(rename = "prototype")]
    Prototype,
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "count_as_rock_for_filtered_deconstruction")]
    CountAsRockForFilteredDeconstruction,
}
