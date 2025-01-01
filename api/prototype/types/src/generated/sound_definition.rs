pub enum SoundDefinition {
    SoundDefinition {
        filename: FileName,
        max_speed: f32,
        max_volume: f32,
        min_speed: f32,
        min_volume: f32,
        modifiers: SoundDefinitionModifiers,
        preload: bool,
        speed: f32,
        volume: f32,
    },
    FileName(FileName),
}
pub enum SoundDefinitionModifiers {
    SoundModifier(Box<SoundModifier>),
    VecSoundModifier(Vec<SoundModifier>),
}
