pub struct PersistentWorldAmbientSoundsDefinitionCrossfade {
    base_: crate::types::Fade,
    order: (
        PersistentWorldAmbientSoundsDefinitionCrossfadeOrder,
        PersistentWorldAmbientSoundsDefinitionCrossfadeOrder,
    ),
}
pub enum PersistentWorldAmbientSoundsDefinitionCrossfadeOrder {
    Wind,
    BaseAmbience,
}
