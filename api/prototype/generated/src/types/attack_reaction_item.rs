#[derive(serde::Deserialize)]
pub struct AttackReactionItem {
    action: crate::types::Trigger,
    damage_type: crate::types::DamageTypeID,
    range: f32,
    reaction_modifier: f32,
}
