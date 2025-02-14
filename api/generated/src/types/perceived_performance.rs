#[derive(Debug, serde::Deserialize)]
pub struct PerceivedPerformance {
    // default: Max double
    maximum: Option<f64>,
    #[serde(default = "default_minimum")]
    minimum: f64,
    #[serde(default = "default_performance_to_activity_rate")]
    performance_to_activity_rate: f64,
}
fn default_minimum() -> f64 {
    0.0
}
fn default_performance_to_activity_rate() -> f64 {
    1.0
}
