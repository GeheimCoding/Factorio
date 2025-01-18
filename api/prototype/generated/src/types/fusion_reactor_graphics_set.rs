#[derive(serde::Deserialize)]
pub struct FusionReactorGraphicsSet {
    connections_graphics: Vec<crate::types::FusionReactorConnectionGraphics>,
    // default: `{1, 1, 1}`
    default_fuel_glow_color: crate::types::Color,
    direction_to_connections_graphics:
        std::collections::HashMap<crate::types::DirectionString, Vec<u8>>,
    fusion_effect_uv_map: crate::types::Sprite,
    light: crate::types::LightDefinition,
    plasma_category: crate::types::NeighbourConnectableConnectionCategory,
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
    structure: crate::types::Sprite4Way,
    #[serde(default = "default_use_fuel_glow_color")]
    use_fuel_glow_color: bool,
    working_light_pictures: crate::types::Sprite4Way,
}
fn default_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
fn default_use_fuel_glow_color() -> bool {
    false
}
