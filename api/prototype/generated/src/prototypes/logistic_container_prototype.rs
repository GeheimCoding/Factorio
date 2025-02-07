#[derive(Debug, serde::Deserialize)]
pub struct LogisticContainerPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::ContainerPrototype,
    animation: Option<crate::types::Animation>,
    animation_sound: Option<crate::types::Sound>,
    landing_location_offset: Option<crate::types::Vector>,
    // overridden by some child
    logistic_mode: Option<LogisticContainerPrototypeLogisticMode>,
    max_logistic_slots: Option<u16>,
    #[serde(default = "default_opened_duration")]
    opened_duration: u8,
    // overridden by some child
    #[serde(default = "default_render_not_in_network_icon")]
    render_not_in_network_icon: bool,
    #[serde(default = "default_trash_inventory_size")]
    trash_inventory_size: crate::types::ItemStackIndex,
    #[serde(default = "default_use_exact_mode")]
    use_exact_mode: bool,
}
#[derive(Debug, serde::Deserialize)]
pub enum LogisticContainerPrototypeLogisticMode {
    #[serde(rename = "active-provider")]
    ActiveProvider,
    #[serde(rename = "passive-provider")]
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
