#[derive(Debug, serde::Deserialize)]
pub struct AttackReactionItem {
    action: Option<crate::types::Trigger>,
    damage_type: Option<crate::types::DamageTypeID>,
    range: f32,
    #[serde(default = "default_reaction_modifier")]
    reaction_modifier: f32,
}
fn default_reaction_modifier() -> f32 {
    0.0
}
