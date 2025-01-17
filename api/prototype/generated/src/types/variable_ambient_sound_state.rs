#[derive(serde::Deserialize)]
pub struct VariableAmbientSoundState {
    end_pause: crate::types::RandomRange,
    layers_properties: Vec<crate::types::VariableAmbientSoundLayerStateProperties>,
    name: String,
    next_state: String,
    next_state_layers_finished_layers: Vec<String>,
    next_state_trigger: crate::types::VariableAmbientSoundNextStateTrigger,
    next_states: Vec<crate::types::VariableAmbientSoundNextStateItem>,
    number_of_enabled_layers: crate::types::RandomRange,
    start_pause: crate::types::RandomRange,
    state_duration_seconds: u32,
    type_: crate::types::VariableAmbientSoundStateType,
}
