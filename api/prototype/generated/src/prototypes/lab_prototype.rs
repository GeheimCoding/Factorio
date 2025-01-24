#[derive(serde::Deserialize)]
pub struct LabPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    // default: All effects except quality are allowed
    allowed_effects: Option<crate::types::EffectTypeLimitation>,
    // default: All module categories are allowed
    allowed_module_categories: Option<Vec<crate::types::ModuleCategoryID>>,
    effect_receiver: Option<crate::types::EffectReceiver>,
    energy_source: crate::types::EnergySource,
    energy_usage: crate::types::Energy,
    frozen_patch: Option<crate::types::Sprite>,
    inputs: Vec<crate::types::ItemID>,
    light: Option<crate::types::LightDefinition>,
    module_slots: Option<crate::types::ItemStackIndex>,
    off_animation: Option<crate::types::Animation>,
    on_animation: Option<crate::types::Animation>,
    #[serde(default = "default_researching_speed")]
    researching_speed: f64,
    #[serde(default = "default_science_pack_drain_rate_percent")]
    science_pack_drain_rate_percent: u8,
    trash_inventory_size: Option<crate::types::ItemStackIndex>,
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
