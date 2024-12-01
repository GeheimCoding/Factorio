pub struct FusionReactorGraphicsSet {
    connections_graphics: Vec<FusionReactorConnectionGraphics>,
    default_fuel_glow_color: Color,
    direction_to_connections_graphics: std::collections::HashMap<DirectionString, Vec<u8>>,
    fusion_effect_uv_map: Sprite,
    light: LightDefinition,
    plasma_category: NeighbourConnectableConnectionCategory,
    render_layer: RenderLayer,
    structure: Sprite4Way,
    use_fuel_glow_color: bool,
    working_light_pictures: Sprite4Way,
}
