pub struct EntityWithHealthPrototype {
    base_: crate::prototypes::EntityPrototype,
    alert_when_damaged: bool,
    attack_reaction: EntityWithHealthPrototypeAttackReaction,
    corpse: EntityWithHealthPrototypeCorpse,
    create_ghost_on_death: bool,
    damaged_trigger_effect: crate::types::TriggerEffect,
    dying_explosion: EntityWithHealthPrototypeDyingExplosion,
    dying_trigger_effect: crate::types::TriggerEffect,
    healing_per_tick: f32,
    hide_resistances: bool,
    integration_patch: crate::types::Sprite4Way,
    integration_patch_render_layer: crate::types::RenderLayer,
    loot: Vec<crate::types::LootItem>,
    max_health: f32,
    overkill_fraction: f32,
    random_corpse_variation: bool,
    repair_sound: crate::types::Sound,
    repair_speed_modifier: f32,
    resistances: Vec<crate::types::Resistance>,
}
#[derive(serde::Deserialize)]
pub enum EntityWithHealthPrototypeAttackReaction {
    #[serde(untagged)]
    AttackReactionItem(Box<crate::types::AttackReactionItem>),
    #[serde(untagged)]
    VecAttackReactionItem(Vec<crate::types::AttackReactionItem>),
}
#[derive(serde::Deserialize)]
pub enum EntityWithHealthPrototypeCorpse {
    #[serde(untagged)]
    EntityID(crate::types::EntityID),
    #[serde(untagged)]
    VecEntityID(Vec<crate::types::EntityID>),
}
#[derive(serde::Deserialize)]
pub enum EntityWithHealthPrototypeDyingExplosion {
    #[serde(untagged)]
    ExplosionDefinition(crate::types::ExplosionDefinition),
    #[serde(untagged)]
    VecExplosionDefinition(Vec<crate::types::ExplosionDefinition>),
}
