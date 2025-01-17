#[derive(serde::Deserialize)]
pub struct TemporaryContainerPrototype {
    base_: crate::prototypes::ContainerPrototype,
    alert_after_time: u32,
    destroy_on_empty: bool,
    time_to_live: u32,
}
