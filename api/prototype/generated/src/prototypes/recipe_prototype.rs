#[derive(serde::Deserialize)]
pub struct RecipePrototype {
    base_: crate::prototypes::Prototype,
    #[serde(default = "default_allow_as_intermediate")]
    allow_as_intermediate: bool,
    #[serde(default = "default_allow_consumption")]
    allow_consumption: bool,
    // default: `{"item-limitation.consumption-effect"}`
    allow_consumption_message: Option<crate::types::LocalisedString>,
    #[serde(default = "default_allow_decomposition")]
    allow_decomposition: bool,
    #[serde(default = "default_allow_inserter_overload")]
    allow_inserter_overload: bool,
    #[serde(default = "default_allow_intermediates")]
    allow_intermediates: bool,
    #[serde(default = "default_allow_pollution")]
    allow_pollution: bool,
    // default: `{"item-limitation.pollution-effect"}`
    allow_pollution_message: Option<crate::types::LocalisedString>,
    #[serde(default = "default_allow_productivity")]
    allow_productivity: bool,
    // default: `{"item-limitation.productivity-effect"}`
    allow_productivity_message: Option<crate::types::LocalisedString>,
    #[serde(default = "default_allow_quality")]
    allow_quality: bool,
    // default: `{"item-limitation.quality-effect"}`
    allow_quality_message: Option<crate::types::LocalisedString>,
    #[serde(default = "default_allow_speed")]
    allow_speed: bool,
    // default: `{"item-limitation.speed-effect"}`
    allow_speed_message: Option<crate::types::LocalisedString>,
    // default: All module categories are allowed
    allowed_module_categories: Option<Vec<crate::types::ModuleCategoryID>>,
    alternative_unlock_methods: Option<Vec<crate::types::TechnologyID>>,
    #[serde(default = "default_always_show_made_in")]
    always_show_made_in: bool,
    #[serde(default = "default_always_show_products")]
    always_show_products: bool,
    #[serde(default = "default_category")]
    category: crate::types::RecipeCategoryID,
    crafting_machine_tint: Option<crate::types::RecipeTints>,
    #[serde(default = "default_emissions_multiplier")]
    emissions_multiplier: f64,
    #[serde(default = "default_enabled")]
    enabled: bool,
    #[serde(default = "default_energy_required")]
    energy_required: f64,
    #[serde(default = "default_hide_from_player_crafting")]
    hide_from_player_crafting: bool,
    // default: unset
    hide_from_signal_gui: Option<bool>,
    #[serde(default = "default_hide_from_stats")]
    hide_from_stats: bool,
    icon: Option<crate::types::FileName>,
    #[serde(default = "default_icon_size")]
    icon_size: crate::types::SpriteSizeType,
    icons: Option<Vec<crate::types::IconData>>,
    ingredients: Option<Vec<crate::types::IngredientPrototype>>,
    main_product: Option<String>,
    #[serde(default = "default_maximum_productivity")]
    maximum_productivity: f64,
    #[serde(default = "default_overload_multiplier")]
    overload_multiplier: u32,
    #[serde(default = "default_preserve_products_in_machine_output")]
    preserve_products_in_machine_output: bool,
    #[serde(default = "default_requester_paste_multiplier")]
    requester_paste_multiplier: u32,
    #[serde(default = "default_result_is_always_fresh")]
    result_is_always_fresh: bool,
    results: Option<Vec<crate::types::ProductPrototype>>,
    #[serde(default = "default_show_amount_in_title")]
    show_amount_in_title: bool,
    surface_conditions: Option<Vec<crate::types::SurfaceCondition>>,
    #[serde(default = "default_unlock_results")]
    unlock_results: bool,
}
fn default_allow_as_intermediate() -> bool {
    true
}
fn default_allow_consumption() -> bool {
    true
}
fn default_allow_decomposition() -> bool {
    true
}
fn default_allow_inserter_overload() -> bool {
    true
}
fn default_allow_intermediates() -> bool {
    true
}
fn default_allow_pollution() -> bool {
    true
}
fn default_allow_productivity() -> bool {
    false
}
fn default_allow_quality() -> bool {
    true
}
fn default_allow_speed() -> bool {
    true
}
fn default_always_show_made_in() -> bool {
    false
}
fn default_always_show_products() -> bool {
    false
}
fn default_category() -> crate::types::RecipeCategoryID {
    String::from("crafting")
}
fn default_emissions_multiplier() -> f64 {
    1.0
}
fn default_enabled() -> bool {
    true
}
fn default_energy_required() -> f64 {
    0.5
}
fn default_hide_from_player_crafting() -> bool {
    false
}
fn default_hide_from_stats() -> bool {
    false
}
fn default_icon_size() -> crate::types::SpriteSizeType {
    64
}
fn default_maximum_productivity() -> f64 {
    3.0
}
fn default_overload_multiplier() -> u32 {
    0
}
fn default_preserve_products_in_machine_output() -> bool {
    false
}
fn default_requester_paste_multiplier() -> u32 {
    30
}
fn default_result_is_always_fresh() -> bool {
    false
}
fn default_show_amount_in_title() -> bool {
    true
}
fn default_unlock_results() -> bool {
    true
}
