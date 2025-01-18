#[derive(serde::Deserialize)]
pub struct AmbientSound {
    name: String,
    planet: crate::types::SpaceLocationID,
    sound: crate::types::Sound,
    track_type: crate::types::AmbientSoundType,
    #[serde(rename = "type")]
    type_: String,
    variable_sound: crate::types::VariableAmbientSoundVariableSound,
    #[serde(default = "default_weight")]
    weight: f64,
}
fn default_weight() -> f64 {
    1.0
}
