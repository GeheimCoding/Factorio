#[derive(Debug, serde::Deserialize)]
pub enum CloudEffectStyle {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "euclidean")]
    Euclidean,
    #[serde(rename = "manhattan")]
    Manhattan,
    #[serde(rename = "euclidean-outside")]
    EuclideanOutside,
    #[serde(rename = "manhattan-outside")]
    ManhattanOutside,
    #[serde(rename = "horizontal-stripe")]
    HorizontalStripe,
    #[serde(rename = "texture")]
    Texture,
    #[serde(rename = "texture-outside")]
    TextureOutside,
}
