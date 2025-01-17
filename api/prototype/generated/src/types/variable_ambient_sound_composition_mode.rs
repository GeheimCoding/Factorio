#[derive(serde::Deserialize)]
pub enum VariableAmbientSoundCompositionMode {
    #[serde(rename = "randomized")]
    Randomized,
    #[serde(rename = "semi_randomized")]
    SemiRandomized,
    #[serde(rename = "shuffled")]
    Shuffled,
    #[serde(rename = "layer_controlled")]
    LayerControlled,
}
