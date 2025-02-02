#[derive(Debug, serde::Deserialize)]
pub struct ResourceEntityPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityPrototype,
    #[serde(default = "default_category")]
    category: crate::types::ResourceCategoryID,
    #[serde(default = "default_cliff_removal_probability")]
    cliff_removal_probability: f64,
    #[serde(default = "default_draw_stateless_visualisation_under_building")]
    draw_stateless_visualisation_under_building: bool,
    driving_sound: Option<crate::types::InterruptibleSound>,
    #[serde(default = "default_effect_animation_period")]
    effect_animation_period: f32,
    #[serde(default = "default_effect_animation_period_deviation")]
    effect_animation_period_deviation: f32,
    #[serde(default = "default_effect_darkness_multiplier")]
    effect_darkness_multiplier: f32,
    #[serde(default = "default_highlight")]
    highlight: bool,
    #[serde(default = "default_infinite")]
    infinite: bool,
    #[serde(default = "default_infinite_depletion_amount")]
    infinite_depletion_amount: u32,
    #[serde(default = "default_map_grid")]
    map_grid: bool,
    #[serde(default = "default_max_effect_alpha")]
    max_effect_alpha: f32,
    #[serde(default = "default_min_effect_alpha")]
    min_effect_alpha: f32,
    #[serde(default = "default_minimum")]
    minimum: u32,
    mining_visualisation_tint: Option<crate::types::Color>,
    #[serde(default = "default_normal")]
    normal: u32,
    #[serde(default = "default_randomize_visual_position")]
    randomize_visual_position: bool,
    #[serde(default = "default_resource_patch_search_radius")]
    resource_patch_search_radius: u32,
    stage_counts: Vec<u32>,
    stages: Option<crate::types::AnimationVariations>,
    stages_effect: Option<crate::types::AnimationVariations>,
    #[serde(default = "default_tree_removal_max_distance")]
    tree_removal_max_distance: f64,
    #[serde(default = "default_tree_removal_probability")]
    tree_removal_probability: f64,
    walking_sound: Option<crate::types::Sound>,
}
fn default_category() -> crate::types::ResourceCategoryID {
    String::from("basic-solid")
}
fn default_cliff_removal_probability() -> f64 {
    1.0
}
fn default_draw_stateless_visualisation_under_building() -> bool {
    true
}
fn default_effect_animation_period() -> f32 {
    0.0
}
fn default_effect_animation_period_deviation() -> f32 {
    0.0
}
fn default_effect_darkness_multiplier() -> f32 {
    1.0
}
fn default_highlight() -> bool {
    false
}
fn default_infinite() -> bool {
    false
}
fn default_infinite_depletion_amount() -> u32 {
    1
}
fn default_map_grid() -> bool {
    true
}
fn default_max_effect_alpha() -> f32 {
    1.0
}
fn default_min_effect_alpha() -> f32 {
    0.0
}
fn default_minimum() -> u32 {
    0
}
fn default_normal() -> u32 {
    1
}
fn default_randomize_visual_position() -> bool {
    true
}
fn default_resource_patch_search_radius() -> u32 {
    3
}
fn default_tree_removal_max_distance() -> f64 {
    0.0
}
fn default_tree_removal_probability() -> f64 {
    0.0
}
