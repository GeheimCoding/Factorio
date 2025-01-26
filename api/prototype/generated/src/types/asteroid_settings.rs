#[derive(Debug, serde::Deserialize)]
pub struct AsteroidSettings {
    max_ray_portals_expanded_per_tick: u32,
    spawning_rate: f64,
}
