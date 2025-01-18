#[derive(serde::Deserialize)]
pub struct AsteroidSpawnPoint {
    #[serde(default = "default_angle_when_stopped")]
    angle_when_stopped: f64,
    probability: f64,
    speed: f64,
}
fn default_angle_when_stopped() -> f64 {
    1.0
}
