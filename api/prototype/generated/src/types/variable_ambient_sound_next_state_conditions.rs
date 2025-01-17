#[derive(serde::Deserialize)]
pub struct VariableAmbientSoundNextStateConditions {
    layer_sample: crate::types::VariableAmbientSoundLayerSample,
    previous_state: String,
    weight: u32,
}
