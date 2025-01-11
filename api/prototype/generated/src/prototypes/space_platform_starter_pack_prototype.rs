pub struct SpacePlatformStarterPackPrototype {
    base_: crate::prototypes::ItemPrototype,
    create_electric_network: bool,
    initial_items: Vec<crate::types::ItemProductPrototype>,
    surface: crate::types::SurfaceID,
    tiles: Vec<crate::types::SpacePlatformTileDefinition>,
    trigger: crate::types::Trigger,
}
