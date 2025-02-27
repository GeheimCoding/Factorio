#[derive(Debug, serde::Deserialize)]
pub struct ItemRequestProxyPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityPrototype,
    #[serde(default = "default_use_target_entity_alert_icon_shift")]
    use_target_entity_alert_icon_shift: bool,
}
fn default_use_target_entity_alert_icon_shift() -> bool {
    true
}
