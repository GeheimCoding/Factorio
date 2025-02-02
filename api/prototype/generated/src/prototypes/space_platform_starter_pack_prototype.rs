#[derive(Debug, serde::Deserialize)]
pub struct SpacePlatformStarterPackPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::ItemPrototype,
    #[serde(default = "default_create_electric_network")]
    create_electric_network: bool,
    initial_items: Option<crate::vec::Vec<crate::types::ItemProductPrototype>>,
    surface: Option<crate::types::SurfaceID>,
    tiles: Option<crate::vec::Vec<crate::types::SpacePlatformTileDefinition>>,
    trigger: Option<crate::types::Trigger>,
}
fn default_create_electric_network() -> bool {
    false
}
