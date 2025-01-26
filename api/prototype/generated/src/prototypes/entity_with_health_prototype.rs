#[derive(Debug, serde::Deserialize)]
pub struct EntityWithHealthPrototype {
    base_: crate::prototypes::EntityPrototype,
    #[serde(default = "default_alert_when_damaged")]
    alert_when_damaged: bool,
    // default: Empty
    attack_reaction: Option<EntityWithHealthPrototypeAttackReaction>,
    // default: Empty
    corpse: Option<EntityWithHealthPrototypeCorpse>,
    #[serde(default = "default_create_ghost_on_death")]
    create_ghost_on_death: bool,
    damaged_trigger_effect: Option<crate::types::TriggerEffect>,
    dying_explosion: Option<EntityWithHealthPrototypeDyingExplosion>,
    dying_trigger_effect: Option<crate::types::TriggerEffect>,
    #[serde(default = "default_healing_per_tick")]
    healing_per_tick: f32,
    #[serde(default = "default_hide_resistances")]
    hide_resistances: bool,
    integration_patch: Option<crate::types::Sprite4Way>,
    #[serde(default = "default_integration_patch_render_layer")]
    integration_patch_render_layer: crate::types::RenderLayer,
    loot: Option<Vec<crate::types::LootItem>>,
    #[serde(default = "default_max_health")]
    max_health: f32,
    #[serde(default = "default_overkill_fraction")]
    overkill_fraction: f32,
    #[serde(default = "default_random_corpse_variation")]
    random_corpse_variation: bool,
    // default: Utility sound defaultManualRepair
    repair_sound: Option<crate::types::Sound>,
    #[serde(default = "default_repair_speed_modifier")]
    repair_speed_modifier: f32,
    resistances: Option<Vec<crate::types::Resistance>>,
}
fn default_alert_when_damaged() -> bool {
    true
}
#[derive(Debug, serde::Deserialize)]
pub enum EntityWithHealthPrototypeAttackReaction {
    #[serde(untagged)]
    AttackReactionItem(Box<crate::types::AttackReactionItem>),
    #[serde(untagged)]
    VecAttackReactionItem(Vec<crate::types::AttackReactionItem>),
}
#[derive(Debug, serde::Deserialize)]
pub enum EntityWithHealthPrototypeCorpse {
    #[serde(untagged)]
    EntityID(crate::types::EntityID),
    #[serde(untagged)]
    VecEntityID(Vec<crate::types::EntityID>),
}
fn default_create_ghost_on_death() -> bool {
    true
}
#[derive(Debug, serde::Deserialize)]
pub enum EntityWithHealthPrototypeDyingExplosion {
    #[serde(untagged)]
    ExplosionDefinition(crate::types::ExplosionDefinition),
    #[serde(untagged)]
    VecExplosionDefinition(Vec<crate::types::ExplosionDefinition>),
}
fn default_healing_per_tick() -> f32 {
    0.0
}
fn default_hide_resistances() -> bool {
    true
}
fn default_integration_patch_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::LowerObject
}
fn default_max_health() -> f32 {
    10.0
}
fn default_overkill_fraction() -> f32 {
    0.1
}
fn default_random_corpse_variation() -> bool {
    false
}
fn default_repair_speed_modifier() -> f32 {
    1.0
}
