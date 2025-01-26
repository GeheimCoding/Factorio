#[derive(Debug, serde::Deserialize)]
pub struct TreePrototype {
    base_: crate::prototypes::EntityWithHealthPrototype,
    colors: Option<Vec<crate::types::Color>>,
    #[serde(default = "default_darkness_of_burnt_tree")]
    darkness_of_burnt_tree: f32,
    #[serde(default = "default_healing_per_tick")]
    healing_per_tick: f32,
    pictures: Option<crate::types::SpriteVariations>,
    stateless_visualisation_variations: Option<Vec<crate::types::StatelessVisualisations>>,
    variation_weights: Option<Vec<f32>>,
    variations: Option<Vec<crate::types::TreeVariation>>,
}
fn default_darkness_of_burnt_tree() -> f32 {
    0.5
}
fn default_healing_per_tick() -> f32 {
    0.0
}
