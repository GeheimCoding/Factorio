#[derive(Debug, serde::Deserialize)]
pub struct BeamTriggerDelivery {
    #[serde(flatten)]
    base_: crate::types::TriggerDeliveryItem,
    #[serde(default = "default_add_to_shooter")]
    add_to_shooter: bool,
    beam: crate::types::EntityID,
    #[serde(default = "default_destroy_with_source_or_target")]
    destroy_with_source_or_target: bool,
    #[serde(default = "default_duration")]
    duration: u32,
    #[serde(default = "default_max_length")]
    max_length: u32,
    source_offset: Option<crate::types::Vector>,
}
fn default_add_to_shooter() -> bool {
    true
}
fn default_destroy_with_source_or_target() -> bool {
    true
}
fn default_duration() -> u32 {
    0
}
fn default_max_length() -> u32 {
    0
}
