#[derive(serde::Deserialize)]
pub enum PersistentWorldAmbientSoundDefinition {
    #[serde(untagged)]
    PersistentWorldAmbientSoundDefinition { sound: crate::types::Sound },
    #[serde(untagged)]
    Sound(crate::types::Sound),
}
