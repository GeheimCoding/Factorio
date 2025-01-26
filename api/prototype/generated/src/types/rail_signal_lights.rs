#[derive(Debug, serde::Deserialize)]
pub struct RailSignalLights {
    blue: Option<crate::types::RailSignalLightDefinition>,
    green: Option<crate::types::RailSignalLightDefinition>,
    red: Option<crate::types::RailSignalLightDefinition>,
    yellow: Option<crate::types::RailSignalLightDefinition>,
}
