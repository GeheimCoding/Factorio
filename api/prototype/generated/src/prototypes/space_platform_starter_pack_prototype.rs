#[derive(Debug, serde::Deserialize)]
pub struct SpacePlatformStarterPackPrototype {
    base_: crate::prototypes::ItemPrototype,
    #[serde(default = "default_create_electric_network")]
    create_electric_network: bool,
    initial_items: Option<Vec<crate::types::ItemProductPrototype>>,
    surface: Option<crate::types::SurfaceID>,
    tiles: Option<Vec<crate::types::SpacePlatformTileDefinition>>,
    trigger: Option<crate::types::Trigger>,
}
fn default_create_electric_network() -> bool {
    false
}
