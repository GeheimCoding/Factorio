#[derive(serde::Deserialize)]
pub struct MapGenPresetPollutionSettings {
    ageing: Option<f64>,
    diffusion_ratio: Option<f64>,
    enabled: Option<bool>,
    enemy_attack_pollution_consumption_modifier: Option<f64>,
    min_pollution_to_damage_trees: Option<f64>,
    pollution_restored_per_tree_damage: Option<f64>,
}
