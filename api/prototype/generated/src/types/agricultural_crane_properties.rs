#[derive(serde::Deserialize)]
pub struct AgriculturalCraneProperties {
    #[serde(default = "default_min_arm_extent")]
    min_arm_extent: f64,
    #[serde(default = "default_min_grappler_extent")]
    min_grappler_extent: f64,
    #[serde(default = "default_operation_angle")]
    operation_angle: f32,
    origin: crate::types::Vector3D,
    parts: Vec<crate::types::CranePart>,
    shadow_direction: crate::types::Vector3D,
    speed: crate::types::AgriculturalCraneSpeed,
    #[serde(default = "default_telescope_default_extention")]
    telescope_default_extention: f64,
}
fn default_min_arm_extent() -> f64 {
    0.0
}
fn default_min_grappler_extent() -> f64 {
    0.2
}
fn default_operation_angle() -> f32 {
    45.0
}
fn default_telescope_default_extention() -> f64 {
    0.5
}
