pub struct SetLogisticRequestTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    entity: crate::types::EntityID,
    logistic_chest_only: bool,
    type_: String,
}
