#[derive(Debug, serde::Deserialize)]
pub struct VariableAmbientSoundState {
    end_pause: Option<crate::types::RandomRange>,
    layers_properties: Option<Vec<crate::types::VariableAmbientSoundLayerStateProperties>>,
    name: String,
    next_state: Option<String>,
    next_state_layers_finished_layers: Option<Vec<String>>,
    next_state_trigger: Option<crate::types::VariableAmbientSoundNextStateTrigger>,
    next_states: Option<Vec<crate::types::VariableAmbientSoundNextStateItem>>,
    number_of_enabled_layers: Option<crate::types::RandomRange>,
    start_pause: Option<crate::types::RandomRange>,
    state_duration_seconds: Option<u32>,
    #[serde(rename = "type")]
    #[serde(default = "default_type_")]
    type_: crate::types::VariableAmbientSoundStateType,
}
fn default_type_() -> crate::types::VariableAmbientSoundStateType {
    crate::types::VariableAmbientSoundStateType::Regular
}
