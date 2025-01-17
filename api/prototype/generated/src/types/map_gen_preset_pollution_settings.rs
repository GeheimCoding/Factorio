#[derive(serde::Deserialize)]
pub struct MapGenPresetPollutionSettings {
    ageing: f64,
    diffusion_ratio: f64,
    enabled: bool,
    enemy_attack_pollution_consumption_modifier: f64,
    min_pollution_to_damage_trees: f64,
    pollution_restored_per_tree_damage: f64,
}
