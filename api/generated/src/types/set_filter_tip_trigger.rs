#[derive(Debug, serde::Deserialize)]
pub struct SetFilterTipTrigger {
    #[serde(flatten)]
    base_: crate::types::CountBasedTipTrigger,
    #[serde(default = "default_consecutive")]
    consecutive: bool,
    entity: Option<crate::types::EntityID>,
    #[serde(default = "default_match_type_only")]
    match_type_only: bool,
}
fn default_consecutive() -> bool {
    false
}
fn default_match_type_only() -> bool {
    false
}
