#[derive(serde::Deserialize)]
pub struct KillTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    damage_type: Option<crate::types::DamageTypeID>,
    entity: Option<crate::types::EntityID>,
    #[serde(default = "default_match_type_only")]
    match_type_only: bool,
    #[serde(rename = "type")]
    type_: String,
}
fn default_match_type_only() -> bool {
    false
}
