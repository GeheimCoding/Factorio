#[derive(Debug, serde::Deserialize)]
pub struct ClusterTriggerItem {
    #[serde(flatten)]
    base_: crate::types::TriggerItem,
    cluster_count: u32,
    distance: f32,
    #[serde(default = "default_distance_deviation")]
    distance_deviation: f32,
}
fn default_distance_deviation() -> f32 {
    0.0
}
