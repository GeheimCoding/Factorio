#[derive(serde::Deserialize)]
pub struct LightProperties {
    // default: {1, 1, 1, 1}
    color: Option<crate::types::Color>,
    // default: {1, 1, 1}
    direction: Option<crate::types::Vector3D>,
}
