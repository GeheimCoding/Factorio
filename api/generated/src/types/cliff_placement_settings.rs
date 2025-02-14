#[derive(Debug, serde::Deserialize)]
pub struct CliffPlacementSettings {
    #[serde(default = "default_cliff_elevation_0")]
    cliff_elevation_0: f32,
    #[serde(default = "default_cliff_elevation_interval")]
    cliff_elevation_interval: f32,
    #[serde(default = "default_cliff_smoothing")]
    cliff_smoothing: f32,
    control: Option<crate::types::AutoplaceControlID>,
    name: Option<crate::types::EntityID>,
    richness: Option<f32>,
}
fn default_cliff_elevation_0() -> f32 {
    10.0
}
fn default_cliff_elevation_interval() -> f32 {
    40.0
}
fn default_cliff_smoothing() -> f32 {
    1.0
}
