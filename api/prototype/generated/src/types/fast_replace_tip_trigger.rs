pub struct FastReplaceTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    match_type_only: bool,
    source: crate::types::EntityID,
    target: crate::types::EntityID,
    type_: String,
}
