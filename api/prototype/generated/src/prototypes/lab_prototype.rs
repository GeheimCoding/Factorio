#[derive(serde::Deserialize)]
pub struct LabPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    allowed_effects: crate::types::EffectTypeLimitation,
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
    researching_speed: f64,
    science_pack_drain_rate_percent: u8,
    trash_inventory_size: crate::types::ItemStackIndex,
    uses_quality_drain_modifier: bool,
}
