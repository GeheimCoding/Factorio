#[derive(serde::Deserialize)]
pub struct SetFilterTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    consecutive: bool,
    entity: crate::types::EntityID,
    match_type_only: bool,
    type_: String,
}
