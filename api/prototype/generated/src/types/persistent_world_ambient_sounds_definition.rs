#[derive(serde::Deserialize)]
pub struct PersistentWorldAmbientSoundsDefinition {
    base_ambience: Option<PersistentWorldAmbientSoundsDefinitionBaseAmbience>,
    crossfade: Option<crate::types::PersistentWorldAmbientSoundsDefinitionCrossfade>,
    semi_persistent: Option<PersistentWorldAmbientSoundsDefinitionSemiPersistent>,
    wind: Option<PersistentWorldAmbientSoundsDefinitionWind>,
}
#[derive(serde::Deserialize)]
pub enum PersistentWorldAmbientSoundsDefinitionBaseAmbience {
    #[serde(untagged)]
    PersistentWorldAmbientSoundDefinition(crate::types::PersistentWorldAmbientSoundDefinition),
    #[serde(untagged)]
    VecPersistentWorldAmbientSoundDefinition(
        Vec<crate::types::PersistentWorldAmbientSoundDefinition>,
    ),
}
#[derive(serde::Deserialize)]
pub enum PersistentWorldAmbientSoundsDefinitionSemiPersistent {
    #[serde(untagged)]
    SemiPersistentWorldAmbientSoundDefinition(
        crate::types::SemiPersistentWorldAmbientSoundDefinition,
    ),
    #[serde(untagged)]
    VecSemiPersistentWorldAmbientSoundDefinition(
        Vec<crate::types::SemiPersistentWorldAmbientSoundDefinition>,
    ),
}
#[derive(serde::Deserialize)]
pub enum PersistentWorldAmbientSoundsDefinitionWind {
    #[serde(untagged)]
    PersistentWorldAmbientSoundDefinition(crate::types::PersistentWorldAmbientSoundDefinition),
    #[serde(untagged)]
    VecPersistentWorldAmbientSoundDefinition(
        Vec<crate::types::PersistentWorldAmbientSoundDefinition>,
    ),
}
