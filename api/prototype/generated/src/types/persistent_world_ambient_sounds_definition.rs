#[derive(Debug, serde::Deserialize)]
pub struct PersistentWorldAmbientSoundsDefinition {
    base_ambience: Option<PersistentWorldAmbientSoundsDefinitionBaseAmbience>,
    crossfade: Option<crate::types::PersistentWorldAmbientSoundsDefinitionCrossfade>,
    semi_persistent: Option<PersistentWorldAmbientSoundsDefinitionSemiPersistent>,
    wind: Option<PersistentWorldAmbientSoundsDefinitionWind>,
}
#[derive(Debug, serde::Deserialize)]
pub enum PersistentWorldAmbientSoundsDefinitionBaseAmbience {
    #[serde(untagged)]
    PersistentWorldAmbientSoundDefinition(Box<crate::types::PersistentWorldAmbientSoundDefinition>),
    #[serde(untagged)]
    VecPersistentWorldAmbientSoundDefinition(
        crate::vec::Vec<crate::types::PersistentWorldAmbientSoundDefinition>,
    ),
}
#[derive(Debug, serde::Deserialize)]
pub enum PersistentWorldAmbientSoundsDefinitionSemiPersistent {
    #[serde(untagged)]
    SemiPersistentWorldAmbientSoundDefinition(
        Box<crate::types::SemiPersistentWorldAmbientSoundDefinition>,
    ),
    #[serde(untagged)]
    VecSemiPersistentWorldAmbientSoundDefinition(
        crate::vec::Vec<crate::types::SemiPersistentWorldAmbientSoundDefinition>,
    ),
}
#[derive(Debug, serde::Deserialize)]
pub enum PersistentWorldAmbientSoundsDefinitionWind {
    #[serde(untagged)]
    PersistentWorldAmbientSoundDefinition(Box<crate::types::PersistentWorldAmbientSoundDefinition>),
    #[serde(untagged)]
    VecPersistentWorldAmbientSoundDefinition(
        crate::vec::Vec<crate::types::PersistentWorldAmbientSoundDefinition>,
    ),
}
