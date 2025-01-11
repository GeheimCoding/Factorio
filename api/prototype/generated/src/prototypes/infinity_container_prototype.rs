pub struct InfinityContainerPrototype {
    erase_contents_when_mined: bool,
    gui_mode: InfinityContainerPrototypeGuiMode,
    inventory_size: crate::types::ItemStackIndex,
    logistic_mode: InfinityContainerPrototypeLogisticMode,
    render_not_in_network_icon: bool,
}
pub enum InfinityContainerPrototypeGuiMode {
    All,
    None,
    Admins,
}
pub enum InfinityContainerPrototypeLogisticMode {
    ActiveProvider,
    PassiveProvider,
    Requester,
    Storage,
    Buffer,
}
