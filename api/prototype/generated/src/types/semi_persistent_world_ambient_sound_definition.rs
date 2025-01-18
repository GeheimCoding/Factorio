#[derive(serde::Deserialize)]
pub enum SemiPersistentWorldAmbientSoundDefinition {
    #[serde(untagged)]
    SemiPersistentWorldAmbientSoundDefinition {
        #[serde(default = "default_delay_mean_seconds")]
        delay_mean_seconds: f32,
        #[serde(default = "default_delay_variance_seconds")]
        delay_variance_seconds: f32,
        sound: crate::types::Sound,
    },
    #[serde(untagged)]
    Sound(crate::types::Sound),
}
fn default_delay_mean_seconds() -> f32 {
    0.0
}
fn default_delay_variance_seconds() -> f32 {
    0.0
}
