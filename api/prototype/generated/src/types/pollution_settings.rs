#[derive(serde::Deserialize)]
pub struct PollutionSettings {
    ageing: f64,
    diffusion_ratio: f64,
    enabled: bool,
    enemy_attack_pollution_consumption_modifier: f64,
    expected_max_per_chunk: f64,
    max_pollution_to_restore_trees: f64,
    min_pollution_to_damage_trees: f64,
    min_to_diffuse: f64,
    min_to_show_per_chunk: f64,
    pollution_per_tree_damage: f64,
    pollution_restored_per_tree_damage: f64,
    pollution_with_max_forest_damage: f64,
}
