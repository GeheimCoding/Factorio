#[derive(serde::Deserialize)]
pub struct CraftingMachinePrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    // default: No effects are allowed
    allowed_effects: crate::types::EffectTypeLimitation,
    // default: All module categories are allowed
    allowed_module_categories: Vec<crate::types::ModuleCategoryID>,
    crafting_categories: Vec<crate::types::RecipeCategoryID>,
    crafting_speed: f64,
    #[serde(default = "default_draw_entity_info_icon_background")]
    draw_entity_info_icon_background: bool,
    effect_receiver: crate::types::EffectReceiver,
    energy_source: crate::types::EnergySource,
    energy_usage: crate::types::Energy,
    #[serde(default = "default_fast_transfer_modules_into_module_slots_only")]
    fast_transfer_modules_into_module_slots_only: bool,
    fluid_boxes: Vec<crate::types::FluidBox>,
    // default: none
    forced_symmetry: crate::types::Mirroring,
    graphics_set: crate::types::CraftingMachineGraphicsSet,
    graphics_set_flipped: crate::types::CraftingMachineGraphicsSet,
    #[serde(default = "default_ignore_output_full")]
    ignore_output_full: bool,
    #[serde(default = "default_match_animation_speed_to_activity")]
    match_animation_speed_to_activity: bool,
    #[serde(default = "default_module_slots")]
    module_slots: crate::types::ItemStackIndex,
    perceived_performance: crate::types::PerceivedPerformance,
    production_health_effect: crate::types::ProductionHealthEffect,
    #[serde(default = "default_return_ingredients_on_change")]
    return_ingredients_on_change: bool,
    #[serde(default = "default_show_recipe_icon")]
    show_recipe_icon: bool,
    #[serde(default = "default_show_recipe_icon_on_map")]
    show_recipe_icon_on_map: bool,
    trash_inventory_size: crate::types::ItemStackIndex,
    vector_to_place_result: crate::types::Vector,
}
fn default_draw_entity_info_icon_background() -> bool {
    true
}
fn default_fast_transfer_modules_into_module_slots_only() -> bool {
    false
}
fn default_ignore_output_full() -> bool {
    false
}
fn default_match_animation_speed_to_activity() -> bool {
    true
}
fn default_module_slots() -> crate::types::ItemStackIndex {
    0
}
fn default_return_ingredients_on_change() -> bool {
    true
}
fn default_show_recipe_icon() -> bool {
    true
}
fn default_show_recipe_icon_on_map() -> bool {
    true
}
