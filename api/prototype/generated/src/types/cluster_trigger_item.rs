#[derive(serde::Deserialize)]
pub struct ClusterTriggerItem {
    base_: crate::types::TriggerItem,
    cluster_count: u32,
    distance: f32,
    distance_deviation: f32,
    type_: String,
}
