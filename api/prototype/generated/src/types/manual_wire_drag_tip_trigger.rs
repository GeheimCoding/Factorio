pub struct ManualWireDragTipTrigger {
    match_type_only: bool,
    source: EntityID,
    target: EntityID,
    type_: ManualWireDrag,
    wire_type: ManualWireDragTipTriggerWireType,
}
pub enum ManualWireDragTipTriggerWireType {
    Red,
    Green,
    Copper,
}
