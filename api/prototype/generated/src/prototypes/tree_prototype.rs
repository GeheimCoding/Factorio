pub struct TreePrototype {
    colors: Vec<crate::types::Color>,
    darkness_of_burnt_tree: f32,
    healing_per_tick: f32,
    pictures: crate::types::SpriteVariations,
    stateless_visualisation_variations: Vec<crate::types::StatelessVisualisations>,
    variation_weights: Vec<f32>,
    variations: Vec<crate::types::TreeVariation>,
}
