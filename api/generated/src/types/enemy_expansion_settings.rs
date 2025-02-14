#[derive(Debug, serde::Deserialize)]
pub struct EnemyExpansionSettings {
    building_coefficient: f64,
    enabled: bool,
    enemy_building_influence_radius: u32,
    friendly_base_influence_radius: u32,
    max_colliding_tiles_coefficient: f64,
    max_expansion_cooldown: u32,
    max_expansion_distance: u32,
    min_expansion_cooldown: u32,
    neighbouring_base_chunk_coefficient: f64,
    neighbouring_chunk_coefficient: f64,
    other_base_coefficient: f64,
    settler_group_max_size: u32,
    settler_group_min_size: u32,
}
