#[derive(serde::Deserialize)]
pub struct TemporaryContainerPrototype {
    base_: crate::prototypes::ContainerPrototype,
    #[serde(default = "default_alert_after_time")]
    alert_after_time: u32,
    #[serde(default = "default_destroy_on_empty")]
    destroy_on_empty: bool,
    #[serde(default = "default_time_to_live")]
    time_to_live: u32,
}
fn default_alert_after_time() -> u32 {
    0
}
fn default_destroy_on_empty() -> bool {
    true
}
fn default_time_to_live() -> u32 {
    0
}
