pub struct PersistentWorldAmbientSoundsDefinition {
    base_ambience: PersistentWorldAmbientSoundsDefinitionBaseAmbience,
    crossfade: PersistentWorldAmbientSoundsDefinitionCrossfade,
    semi_persistent: PersistentWorldAmbientSoundsDefinitionSemiPersistent,
    wind: PersistentWorldAmbientSoundsDefinitionWind,
}
pub enum PersistentWorldAmbientSoundsDefinitionBaseAmbience {
    PersistentWorldAmbientSoundDefinition(PersistentWorldAmbientSoundDefinition),
    VecPersistentWorldAmbientSoundDefinition(Vec<PersistentWorldAmbientSoundDefinition>),
}
pub enum PersistentWorldAmbientSoundsDefinitionSemiPersistent {
    SemiPersistentWorldAmbientSoundDefinition(SemiPersistentWorldAmbientSoundDefinition),
    VecSemiPersistentWorldAmbientSoundDefinition(Vec<SemiPersistentWorldAmbientSoundDefinition>),
}
pub enum PersistentWorldAmbientSoundsDefinitionWind {
    PersistentWorldAmbientSoundDefinition(PersistentWorldAmbientSoundDefinition),
    VecPersistentWorldAmbientSoundDefinition(Vec<PersistentWorldAmbientSoundDefinition>),
}
