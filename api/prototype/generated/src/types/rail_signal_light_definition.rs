#[derive(serde::Deserialize)]
pub struct RailSignalLightDefinition {
    light: crate::types::LightDefinition,
    shift: crate::types::Vector,
}
