#[derive(serde::Deserialize)]
pub struct AgriculturalCraneProperties {
    min_arm_extent: f64,
    min_grappler_extent: f64,
    operation_angle: f32,
    origin: crate::types::Vector3D,
    parts: Vec<crate::types::CranePart>,
    shadow_direction: crate::types::Vector3D,
    speed: crate::types::AgriculturalCraneSpeed,
    telescope_default_extention: f64,
}
