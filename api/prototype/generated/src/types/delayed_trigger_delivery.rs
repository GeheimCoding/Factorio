pub struct DelayedTriggerDelivery {
    base_: crate::types::TriggerDeliveryItem,
    delayed_trigger: crate::types::ActiveTriggerID,
    type_: String,
}
