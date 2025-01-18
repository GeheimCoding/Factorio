#[derive(serde::Deserialize)]
pub struct ClusterTriggerItem {
    base_: crate::types::TriggerItem,
    cluster_count: u32,
    distance: f32,
    #[serde(default = "default_distance_deviation")]
    distance_deviation: f32,
    #[serde(rename = "type")]
    type_: String,
}
fn default_distance_deviation() -> f32 {
    0.0
}
