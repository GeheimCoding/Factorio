pub struct PersistentWorldAmbientSoundsDefinitionCrossfade {
    order: (
        PersistentWorldAmbientSoundsDefinitionCrossfadeOrder,
        PersistentWorldAmbientSoundsDefinitionCrossfadeOrder,
    ),
}
pub enum PersistentWorldAmbientSoundsDefinitionCrossfadeOrder {
    Wind,
    BaseAmbience,
}