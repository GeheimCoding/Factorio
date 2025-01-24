#[derive(serde::Deserialize)]
pub struct LogisticContainerPrototype {
    base_: crate::prototypes::ContainerPrototype,
    animation: Option<crate::types::Animation>,
    animation_sound: Option<crate::types::Sound>,
    landing_location_offset: Option<crate::types::Vector>,
    logistic_mode: LogisticContainerPrototypeLogisticMode,
    max_logistic_slots: Option<u16>,
    #[serde(default = "default_opened_duration")]
    opened_duration: u8,
    #[serde(default = "default_render_not_in_network_icon")]
    render_not_in_network_icon: bool,
    #[serde(default = "default_trash_inventory_size")]
    trash_inventory_size: crate::types::ItemStackIndex,
    #[serde(default = "default_use_exact_mode")]
    use_exact_mode: bool,
}
#[derive(serde::Deserialize)]
pub enum LogisticContainerPrototypeLogisticMode {
    #[serde(rename = "active_provider")]
    ActiveProvider,
    #[serde(rename = "passive_provider")]
    PassiveProvider,
    #[serde(rename = "requester")]
    Requester,
    #[serde(rename = "storage")]
    Storage,
    #[serde(rename = "buffer")]
    Buffer,
}
fn default_opened_duration() -> u8 {
    0
}
fn default_render_not_in_network_icon() -> bool {
    true
}
fn default_trash_inventory_size() -> crate::types::ItemStackIndex {
    0
}
fn default_use_exact_mode() -> bool {
    false
}
