#[derive(Debug, serde::Deserialize)]
pub struct FusionReactorGraphicsSet {
    connections_graphics: Option<Vec<crate::types::FusionReactorConnectionGraphics>>,
    // default: `{1, 1, 1}`
    default_fuel_glow_color: Option<crate::types::Color>,
    direction_to_connections_graphics:
        Option<std::collections::HashMap<crate::types::DirectionString, Vec<u8>>>,
    fusion_effect_uv_map: Option<crate::types::Sprite>,
    light: Option<crate::types::LightDefinition>,
    plasma_category: crate::types::NeighbourConnectableConnectionCategory,
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
    structure: Option<crate::types::Sprite4Way>,
    #[serde(default = "default_use_fuel_glow_color")]
    use_fuel_glow_color: bool,
    working_light_pictures: Option<crate::types::Sprite4Way>,
}
fn default_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
fn default_use_fuel_glow_color() -> bool {
    false
}
