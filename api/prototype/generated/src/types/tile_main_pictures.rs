#[derive(Debug, serde::Deserialize)]
pub struct TileMainPictures {
    base_: crate::types::TileSpriteLayout,
    #[serde(default = "default_probability")]
    probability: f64,
    size: u32,
    weights: Option<Vec<f64>>,
}
fn default_probability() -> f64 {
    1.0
}
