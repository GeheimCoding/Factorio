#[derive(serde::Deserialize)]
pub enum SoundDefinition {
    #[serde(untagged)]
    SoundDefinition {
        filename: crate::types::FileName,
        max_speed: f32,
        max_volume: f32,
        min_speed: f32,
        min_volume: f32,
        modifiers: SoundDefinitionModifiers,
        preload: bool,
        speed: f32,
        volume: f32,
    },
    #[serde(untagged)]
    FileName(crate::types::FileName),
}
#[derive(serde::Deserialize)]
pub enum SoundDefinitionModifiers {
    #[serde(untagged)]
    SoundModifier(Box<crate::types::SoundModifier>),
    #[serde(untagged)]
    VecSoundModifier(Vec<crate::types::SoundModifier>),
}
