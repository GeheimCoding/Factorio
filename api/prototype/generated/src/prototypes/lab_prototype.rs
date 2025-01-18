#[derive(serde::Deserialize)]
pub struct LabPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    // default: All effects except quality are allowed
    allowed_effects: crate::types::EffectTypeLimitation,
    // default: All module categories are allowed
    allowed_module_categories: Vec<crate::types::ModuleCategoryID>,
    effect_receiver: crate::types::EffectReceiver,
    energy_source: crate::types::EnergySource,
    energy_usage: crate::types::Energy,
    frozen_patch: crate::types::Sprite,
    inputs: Vec<crate::types::ItemID>,
    light: crate::types::LightDefinition,
    module_slots: crate::types::ItemStackIndex,
    off_animation: crate::types::Animation,
    on_animation: crate::types::Animation,
    #[serde(default = "default_researching_speed")]
    researching_speed: f64,
    #[serde(default = "default_science_pack_drain_rate_percent")]
    science_pack_drain_rate_percent: u8,
    trash_inventory_size: crate::types::ItemStackIndex,
    #[serde(default = "default_uses_quality_drain_modifier")]
    uses_quality_drain_modifier: bool,
}
fn default_researching_speed() -> f64 {
    1.0
}
fn default_science_pack_drain_rate_percent() -> u8 {
    100
}
fn default_uses_quality_drain_modifier() -> bool {
    false
}
