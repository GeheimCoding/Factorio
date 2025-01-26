#[derive(Debug, serde::Deserialize)]
pub struct AggregationSpecification {
    #[serde(default = "default_count_already_playing")]
    count_already_playing: bool,
    max_count: u32,
    #[serde(default = "default_priority")]
    priority: AggregationSpecificationPriority,
    #[serde(default = "default_progress_threshold")]
    progress_threshold: f32,
    remove: bool,
    #[serde(default = "default_volume_reduction_rate")]
    volume_reduction_rate: f32,
}
fn default_count_already_playing() -> bool {
    false
}
#[derive(Debug, serde::Deserialize)]
pub enum AggregationSpecificationPriority {
    #[serde(rename = "closest")]
    Closest,
    #[serde(rename = "farthest")]
    Farthest,
    #[serde(rename = "newest")]
    Newest,
    #[serde(rename = "oldest")]
    Oldest,
}
fn default_priority() -> AggregationSpecificationPriority {
    AggregationSpecificationPriority::Closest
}
fn default_progress_threshold() -> f32 {
    1.0
}
fn default_volume_reduction_rate() -> f32 {
    2.0
}
