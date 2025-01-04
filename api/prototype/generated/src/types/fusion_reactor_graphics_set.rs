pub struct FusionReactorGraphicsSet {
    connections_graphics: Vec<crate::types::FusionReactorConnectionGraphics>,
    default_fuel_glow_color: crate::types::Color,
    direction_to_connections_graphics:
        std::collections::HashMap<crate::types::DirectionString, Vec<u8>>,
    fusion_effect_uv_map: crate::types::Sprite,
    light: crate::types::LightDefinition,
    plasma_category: crate::types::NeighbourConnectableConnectionCategory,
    render_layer: crate::types::RenderLayer,
    structure: crate::types::Sprite4Way,
    use_fuel_glow_color: bool,
    working_light_pictures: crate::types::Sprite4Way,
}
