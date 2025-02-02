#[derive(Debug, serde::Deserialize)]
pub struct CharacterPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityWithOwnerPrototype,
    animations: crate::vec::Vec<crate::types::CharacterArmorAnimation>,
    build_distance: u32,
    character_corpse: Option<crate::types::EntityID>,
    crafting_categories: Option<crate::vec::Vec<crate::types::RecipeCategoryID>>,
    damage_hit_tint: crate::types::Color,
    distance_per_frame: f64,
    drop_item_distance: u32,
    eat: crate::types::Sound,
    #[serde(default = "default_enter_vehicle_distance")]
    enter_vehicle_distance: f64,
    #[serde(default = "default_flying_bob_speed")]
    flying_bob_speed: f32,
    flying_collision_mask: Option<crate::types::CollisionMaskConnector>,
    footprint_particles: Option<crate::vec::Vec<crate::types::FootprintParticle>>,
    footstep_particle_triggers: Option<crate::types::FootstepTriggerEffectList>,
    #[serde(default = "default_grounded_landing_search_radius")]
    grounded_landing_search_radius: f64,
    #[serde(default = "default_guns_inventory_size")]
    guns_inventory_size: crate::types::ItemStackIndex,
    #[serde(default = "default_has_belt_immunity")]
    has_belt_immunity: bool,
    heartbeat: crate::types::Sound,
    inventory_size: crate::types::ItemStackIndex,
    #[serde(default = "default_is_military_target")]
    is_military_target: bool,
    item_pickup_distance: f64,
    left_footprint_frames: Option<crate::vec::Vec<f32>>,
    left_footprint_offset: Option<crate::types::Vector>,
    light: Option<crate::types::LightDefinition>,
    loot_pickup_distance: f64,
    maximum_corner_sliding_distance: f64,
    mining_categories: Option<crate::vec::Vec<crate::types::ResourceCategoryID>>,
    mining_speed: f64,
    mining_with_tool_particles_animation_positions: crate::vec::Vec<f32>,
    moving_sound_animation_positions: crate::vec::Vec<f32>,
    reach_distance: u32,
    reach_resource_distance: f64,
    #[serde(default = "default_respawn_time")]
    respawn_time: u32,
    right_footprint_frames: Option<crate::vec::Vec<f32>>,
    right_footprint_offset: Option<crate::types::Vector>,
    running_sound_animation_positions: crate::vec::Vec<f32>,
    running_speed: f64,
    synced_footstep_particle_triggers: Option<crate::types::FootstepTriggerEffectList>,
    ticks_to_keep_aiming_direction: u32,
    ticks_to_keep_gun: u32,
    ticks_to_stay_in_combat: u32,
    #[serde(default = "default_tool_attack_distance")]
    tool_attack_distance: f64,
    tool_attack_result: Option<crate::types::Trigger>,
}
fn default_enter_vehicle_distance() -> f64 {
    3.0
}
fn default_flying_bob_speed() -> f32 {
    1.0
}
fn default_grounded_landing_search_radius() -> f64 {
    0.0
}
fn default_guns_inventory_size() -> crate::types::ItemStackIndex {
    3
}
fn default_has_belt_immunity() -> bool {
    false
}
fn default_is_military_target() -> bool {
    true
}
fn default_respawn_time() -> u32 {
    10
}
fn default_tool_attack_distance() -> f64 {
    1.5
}
