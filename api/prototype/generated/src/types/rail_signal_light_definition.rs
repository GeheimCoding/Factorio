#[derive(Debug, serde::Deserialize)]
pub struct RailSignalLightDefinition {
    light: crate::types::LightDefinition,
    shift: Option<crate::types::Vector>,
}
