#[derive(serde::Deserialize)]
pub struct MapGenPresetEnemyExpansionSettings {
    enabled: bool,
    max_expansion_cooldown: u32,
    max_expansion_distance: u32,
    min_expansion_cooldown: u32,
    settler_group_max_size: u32,
    settler_group_min_size: u32,
}
