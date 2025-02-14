#[derive(Debug, serde::Deserialize)]
pub struct MapGenPresetEnemyExpansionSettings {
    enabled: Option<bool>,
    max_expansion_cooldown: Option<u32>,
    max_expansion_distance: Option<u32>,
    min_expansion_cooldown: Option<u32>,
    settler_group_max_size: Option<u32>,
    settler_group_min_size: Option<u32>,
}
