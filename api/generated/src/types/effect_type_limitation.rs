#[derive(Debug, serde::Deserialize)]
pub enum EffectTypeLimitation {
    #[serde(untagged)]
    EffectTypeLimitationVariants(EffectTypeLimitationVariants),
    #[serde(untagged)]
    VecEffectTypeLimitationVariants(crate::vec::Vec<EffectTypeLimitationVariants>),
}
#[derive(Debug, serde::Deserialize)]
pub enum EffectTypeLimitationVariants {
    #[serde(rename = "speed")]
    Speed,
    #[serde(rename = "productivity")]
    Productivity,
    #[serde(rename = "consumption")]
    Consumption,
    #[serde(rename = "pollution")]
    Pollution,
    #[serde(rename = "quality")]
    Quality,
}
