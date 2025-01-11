pub struct KillTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    damage_type: crate::types::DamageTypeID,
    entity: crate::types::EntityID,
    match_type_only: bool,
    type_: String,
}
