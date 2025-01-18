#[derive(serde::Deserialize)]
pub struct CircuitConnectorLayer {
    #[serde(default = "default_east")]
    east: crate::types::RenderLayer,
    #[serde(default = "default_north")]
    north: crate::types::RenderLayer,
    #[serde(default = "default_south")]
    south: crate::types::RenderLayer,
    #[serde(default = "default_west")]
    west: crate::types::RenderLayer,
}
fn default_east() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
fn default_north() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
fn default_south() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
fn default_west() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
