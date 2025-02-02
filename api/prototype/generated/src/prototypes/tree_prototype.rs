#[derive(Debug, serde::Deserialize)]
pub struct TreePrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityWithHealthPrototype,
    colors: Option<crate::vec::Vec<crate::types::Color>>,
    #[serde(default = "default_darkness_of_burnt_tree")]
    darkness_of_burnt_tree: f32,
    #[serde(default = "default_healing_per_tick")]
    healing_per_tick: f32,
    pictures: Option<crate::types::SpriteVariations>,
    stateless_visualisation_variations:
        Option<crate::vec::Vec<crate::types::StatelessVisualisations>>,
    variation_weights: Option<crate::vec::Vec<f32>>,
    variations: Option<crate::vec::Vec<crate::types::TreeVariation>>,
}
fn default_darkness_of_burnt_tree() -> f32 {
    0.5
}
fn default_healing_per_tick() -> f32 {
    0.0
}
