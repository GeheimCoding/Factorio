#[derive(Debug, serde::Deserialize)]
pub struct VariableAmbientSoundNextStateConditions {
    layer_sample: Option<crate::types::VariableAmbientSoundLayerSample>,
    previous_state: Option<String>,
    weight: u32,
}
