pub struct AggregationSpecification {
    count_already_playing: bool,
    max_count: u32,
    priority: AggregationSpecificationPriority,
    progress_threshold: f32,
    remove: bool,
    volume_reduction_rate: f32,
}
#[derive(serde::Deserialize)]
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
