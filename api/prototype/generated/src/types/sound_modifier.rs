#[derive(serde::Deserialize)]
pub struct SoundModifier {
    #[serde(rename = "type")]
    type_: crate::types::SoundModifierType,
    volume_multiplier: f32,
}
