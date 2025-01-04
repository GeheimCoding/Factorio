pub enum SoundDefinition {
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
    FileName(crate::types::FileName),
}
pub enum SoundDefinitionModifiers {
    SoundModifier(Box<crate::types::SoundModifier>),
    VecSoundModifier(Vec<crate::types::SoundModifier>),
}
