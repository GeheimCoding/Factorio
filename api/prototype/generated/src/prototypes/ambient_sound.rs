#[derive(serde::Deserialize)]
pub struct AmbientSound {
    name: String,
    planet: crate::types::SpaceLocationID,
    sound: crate::types::Sound,
    track_type: crate::types::AmbientSoundType,
    #[serde(rename = "type")]
    type_: String,
    variable_sound: crate::types::VariableAmbientSoundVariableSound,
    weight: f64,
}
