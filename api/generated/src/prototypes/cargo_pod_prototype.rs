#[derive(Debug, serde::Deserialize)]
pub struct CargoPodPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityWithOwnerPrototype,
    default_graphic: Option<crate::types::ProcessionGraphic>,
    default_shadow_graphic: Option<crate::types::ProcessionGraphic>,
    inventory_size: crate::types::ItemStackIndex,
    procession_audio_catalogue: Option<crate::types::ProcessionAudioCatalogue>,
    procession_graphic_catalogue: Option<crate::types::ProcessionGraphicCatalogue>,
    shadow_slave_entity: Option<crate::types::EntityID>,
    spawned_container: crate::types::EntityID,
}
