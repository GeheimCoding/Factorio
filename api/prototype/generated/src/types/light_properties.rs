#[derive(serde::Deserialize)]
pub struct LightProperties {
    // default: {1, 1, 1, 1}
    color: crate::types::Color,
    // default: {1, 1, 1}
    direction: crate::types::Vector3D,
}
