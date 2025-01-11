pub struct LogisticContainerPrototype {
    animation: crate::types::Animation,
    animation_sound: crate::types::Sound,
    landing_location_offset: crate::types::Vector,
    logistic_mode: LogisticContainerPrototypeLogisticMode,
    max_logistic_slots: u16,
    opened_duration: u8,
    render_not_in_network_icon: bool,
    trash_inventory_size: crate::types::ItemStackIndex,
    use_exact_mode: bool,
}
pub enum LogisticContainerPrototypeLogisticMode {
    ActiveProvider,
    PassiveProvider,
    Requester,
    Storage,
    Buffer,
}
