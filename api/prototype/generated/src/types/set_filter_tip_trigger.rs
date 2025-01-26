#[derive(Debug, serde::Deserialize)]
pub struct SetFilterTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    #[serde(default = "default_consecutive")]
    consecutive: bool,
    entity: Option<crate::types::EntityID>,
    #[serde(default = "default_match_type_only")]
    match_type_only: bool,
    #[serde(rename = "type")]
    type_: String,
}
fn default_consecutive() -> bool {
    false
}
fn default_match_type_only() -> bool {
    false
}
