pub struct AgriculturalCraneProperties {
    min_arm_extent: f64,
    min_grappler_extent: f64,
    operation_angle: f32,
    origin: Vector3D,
    parts: Vec<CranePart>,
    shadow_direction: Vector3D,
    speed: AgriculturalCraneSpeed,
    telescope_default_extention: f64,
}
