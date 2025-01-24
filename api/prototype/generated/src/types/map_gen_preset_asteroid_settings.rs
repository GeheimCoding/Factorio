#[derive(serde::Deserialize)]
pub struct MapGenPresetAsteroidSettings {
    max_ray_portals_expanded_per_tick: Option<u32>,
    spawning_rate: Option<f64>,
}
