pub struct AggregationSpecification {
    count_already_playing: bool,
    max_count: u32,
    priority: AggregationSpecificationPriority,
    progress_threshold: f32,
    remove: bool,
    volume_reduction_rate: f32,
}
pub enum AggregationSpecificationPriority {
    Closest,
    Farthest,
    Newest,
    Oldest,
}
