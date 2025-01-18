#[derive(serde::Deserialize)]
pub struct TreePrototype {
    base_: crate::prototypes::EntityWithHealthPrototype,
    colors: Vec<crate::types::Color>,
    #[serde(default = "default_darkness_of_burnt_tree")]
    darkness_of_burnt_tree: f32,
    #[serde(default = "default_healing_per_tick")]
    healing_per_tick: f32,
    pictures: crate::types::SpriteVariations,
    stateless_visualisation_variations: Vec<crate::types::StatelessVisualisations>,
    variation_weights: Vec<f32>,
    variations: Vec<crate::types::TreeVariation>,
}
fn default_darkness_of_burnt_tree() -> f32 {
    0.5
}
fn default_healing_per_tick() -> f32 {
    0.0
}
