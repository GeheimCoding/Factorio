#[derive(serde::Deserialize)]
pub struct AttackReactionItem {
    action: crate::types::Trigger,
    damage_type: crate::types::DamageTypeID,
    range: f32,
    #[serde(default = "default_reaction_modifier")]
    reaction_modifier: f32,
}
fn default_reaction_modifier() -> f32 {
    0.0
}
