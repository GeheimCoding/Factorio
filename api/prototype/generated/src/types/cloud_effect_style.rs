#[derive(Debug, serde::Deserialize)]
pub enum CloudEffectStyle {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "euclidean")]
    Euclidean,
    #[serde(rename = "manhattan")]
    Manhattan,
    #[serde(rename = "euclidean_outside")]
    EuclideanOutside,
    #[serde(rename = "manhattan_outside")]
    ManhattanOutside,
    #[serde(rename = "horizontal_stripe")]
    HorizontalStripe,
    #[serde(rename = "texture")]
    Texture,
    #[serde(rename = "texture_outside")]
    TextureOutside,
}
