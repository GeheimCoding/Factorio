#[derive(serde::Deserialize)]
pub enum SemiPersistentWorldAmbientSoundDefinition {
    #[serde(untagged)]
    SemiPersistentWorldAmbientSoundDefinition {
        delay_mean_seconds: f32,
        delay_variance_seconds: f32,
        sound: crate::types::Sound,
    },
    #[serde(untagged)]
    Sound(crate::types::Sound),
}
