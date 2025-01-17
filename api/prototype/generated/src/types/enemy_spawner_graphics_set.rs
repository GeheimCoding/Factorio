#[derive(serde::Deserialize)]
pub struct EnemySpawnerGraphicsSet {
    animations: crate::types::AnimationVariations,
    integration: crate::types::SpriteVariations,
    random_animation_offset: bool,
    underwater_animations: crate::types::AnimationVariations,
    underwater_layer_offset: i8,
    water_effect_map_animations: crate::types::AnimationVariations,
}
