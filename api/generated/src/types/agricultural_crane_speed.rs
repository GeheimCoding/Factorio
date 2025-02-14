#[derive(Debug, serde::Deserialize)]
pub struct AgriculturalCraneSpeed {
    arm: crate::types::AgriculturalCraneSpeedArm,
    grappler: crate::types::AgriculturalCraneSpeedGrappler,
}
