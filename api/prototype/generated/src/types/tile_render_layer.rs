#[derive(serde::Deserialize)]
pub enum TileRenderLayer {
    #[serde(rename = "zero")]
    Zero,
    #[serde(rename = "water")]
    Water,
    #[serde(rename = "water_overlay")]
    WaterOverlay,
    #[serde(rename = "ground_natural")]
    GroundNatural,
    #[serde(rename = "ground_artificial")]
    GroundArtificial,
    #[serde(rename = "top")]
    Top,
}
