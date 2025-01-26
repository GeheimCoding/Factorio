#[derive(Debug, serde::Deserialize)]
pub enum EffectVariation {
    #[serde(rename = "lava")]
    Lava,
    #[serde(rename = "wetland_water")]
    WetlandWater,
    #[serde(rename = "oil")]
    Oil,
    #[serde(rename = "water")]
    Water,
}
