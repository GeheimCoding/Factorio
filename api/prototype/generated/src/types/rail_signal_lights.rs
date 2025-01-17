#[derive(serde::Deserialize)]
pub struct RailSignalLights {
    blue: crate::types::RailSignalLightDefinition,
    green: crate::types::RailSignalLightDefinition,
    red: crate::types::RailSignalLightDefinition,
    yellow: crate::types::RailSignalLightDefinition,
}
