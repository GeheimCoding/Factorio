pub struct VariableAmbientSoundState {
    end_pause: RandomRange,
    layers_properties: Vec<VariableAmbientSoundLayerStateProperties>,
    name: String,
    next_state: String,
    next_state_layers_finished_layers: Vec<String>,
    next_state_trigger: VariableAmbientSoundNextStateTrigger,
    next_states: Vec<VariableAmbientSoundNextStateItem>,
    number_of_enabled_layers: RandomRange,
    start_pause: RandomRange,
    state_duration_seconds: u32,
    type_: VariableAmbientSoundStateType,
}
