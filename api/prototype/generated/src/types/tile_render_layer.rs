#[derive(Debug, serde::Deserialize)]
pub enum TileRenderLayer {
    #[serde(rename = "zero")]
    Zero,
    #[serde(rename = "water")]
    Water,
    #[serde(rename = "water-overlay")]
    WaterOverlay,
    #[serde(rename = "ground-natural")]
    GroundNatural,
    #[serde(rename = "ground-artificial")]
    GroundArtificial,
    #[serde(rename = "top")]
    Top,
}
