#[derive(Debug, serde::Deserialize)]
pub struct PersistentWorldAmbientSoundsDefinitionCrossfade {
    base_: crate::types::Fade,
    order: (
        PersistentWorldAmbientSoundsDefinitionCrossfadeOrder,
        PersistentWorldAmbientSoundsDefinitionCrossfadeOrder,
    ),
}
#[derive(Debug, serde::Deserialize)]
pub enum PersistentWorldAmbientSoundsDefinitionCrossfadeOrder {
    #[serde(rename = "wind")]
    Wind,
    #[serde(rename = "base_ambience")]
    BaseAmbience,
}
