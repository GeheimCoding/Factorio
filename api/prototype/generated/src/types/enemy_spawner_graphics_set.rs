#[derive(serde::Deserialize)]
pub struct EnemySpawnerGraphicsSet {
    animations: Option<crate::types::AnimationVariations>,
    integration: Option<crate::types::SpriteVariations>,
    #[serde(default = "default_random_animation_offset")]
    random_animation_offset: bool,
    underwater_animations: Option<crate::types::AnimationVariations>,
    #[serde(default = "default_underwater_layer_offset")]
    underwater_layer_offset: i8,
    water_effect_map_animations: Option<crate::types::AnimationVariations>,
}
fn default_random_animation_offset() -> bool {
    true
}
fn default_underwater_layer_offset() -> i8 {
    1
}
