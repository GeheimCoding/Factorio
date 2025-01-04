pub struct PersistentWorldAmbientSoundsDefinition {
    base_ambience: PersistentWorldAmbientSoundsDefinitionBaseAmbience,
    crossfade: crate::types::PersistentWorldAmbientSoundsDefinitionCrossfade,
    semi_persistent: PersistentWorldAmbientSoundsDefinitionSemiPersistent,
    wind: PersistentWorldAmbientSoundsDefinitionWind,
}
pub enum PersistentWorldAmbientSoundsDefinitionBaseAmbience {
    PersistentWorldAmbientSoundDefinition(crate::types::PersistentWorldAmbientSoundDefinition),
    VecPersistentWorldAmbientSoundDefinition(
        Vec<crate::types::PersistentWorldAmbientSoundDefinition>,
    ),
}
pub enum PersistentWorldAmbientSoundsDefinitionSemiPersistent {
    SemiPersistentWorldAmbientSoundDefinition(
        crate::types::SemiPersistentWorldAmbientSoundDefinition,
    ),
    VecSemiPersistentWorldAmbientSoundDefinition(
        Vec<crate::types::SemiPersistentWorldAmbientSoundDefinition>,
    ),
}
pub enum PersistentWorldAmbientSoundsDefinitionWind {
    PersistentWorldAmbientSoundDefinition(crate::types::PersistentWorldAmbientSoundDefinition),
    VecPersistentWorldAmbientSoundDefinition(
        Vec<crate::types::PersistentWorldAmbientSoundDefinition>,
    ),
}
