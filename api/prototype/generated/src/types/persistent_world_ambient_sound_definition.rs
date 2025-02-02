#[derive(Debug, serde::Deserialize)]
pub enum PersistentWorldAmbientSoundDefinition {
    #[serde(untagged)]
    PersistentWorldAmbientSoundDefinition { sound: Box<crate::types::Sound> },
    #[serde(untagged)]
    Sound(Box<crate::types::Sound>),
}
