#[derive(serde::Deserialize)]
pub struct VariableAmbientSoundNextStateItem {
    conditions: crate::types::VariableAmbientSoundNextStateConditions,
    state: String,
}
