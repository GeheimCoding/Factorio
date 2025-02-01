#[derive(Debug, serde::Deserialize)]
pub enum VariableAmbientSoundCompositionMode {
    #[serde(rename = "randomized")]
    Randomized,
    #[serde(rename = "semi-randomized")]
    SemiRandomized,
    #[serde(rename = "shuffled")]
    Shuffled,
    #[serde(rename = "layer-controlled")]
    LayerControlled,
}
