pub enum SemiPersistentWorldAmbientSoundDefinition {
    SemiPersistentWorldAmbientSoundDefinition {
        delay_mean_seconds: f32,
        delay_variance_seconds: f32,
        sound: crate::types::Sound,
    },
    Sound(crate::types::Sound),
}
