#[derive(serde::Deserialize)]
pub struct DelayedActiveTriggerPrototype {
    base_: crate::prototypes::ActiveTriggerPrototype,
    action: crate::types::Trigger,
    #[serde(default = "default_cancel_when_source_is_destroyed")]
    cancel_when_source_is_destroyed: bool,
    delay: u32,
    #[serde(default = "default_repeat_count")]
    repeat_count: u32,
    // default: Value of `delay`
    repeat_delay: u32,
}
fn default_cancel_when_source_is_destroyed() -> bool {
    false
}
fn default_repeat_count() -> u32 {
    0
}
