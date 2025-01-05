pub struct ManualWireDragTipTrigger {
    match_type_only: bool,
    source: crate::types::EntityID,
    target: crate::types::EntityID,
    type_: String,
    wire_type: ManualWireDragTipTriggerWireType,
}
pub enum ManualWireDragTipTriggerWireType {
    Red,
    Green,
    Copper,
}
