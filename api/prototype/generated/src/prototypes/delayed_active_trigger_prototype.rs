pub struct DelayedActiveTriggerPrototype {
    base_: crate::prototypes::ActiveTriggerPrototype,
    action: crate::types::Trigger,
    cancel_when_source_is_destroyed: bool,
    delay: u32,
    repeat_count: u32,
    repeat_delay: u32,
}
