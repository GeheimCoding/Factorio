#[derive(serde::Deserialize)]
pub struct SpaceConnectionPrototype {
    base_: crate::prototypes::Prototype,
    asteroid_spawn_definitions: Vec<crate::types::SpaceConnectionAsteroidSpawnDefinition>,
    from: crate::types::SpaceLocationID,
    icon: crate::types::FileName,
    #[serde(default = "default_icon_size")]
    icon_size: crate::types::SpriteSizeType,
    icons: Vec<crate::types::IconData>,
    #[serde(default = "default_length")]
    length: u32,
    to: crate::types::SpaceLocationID,
}
fn default_icon_size() -> crate::types::SpriteSizeType {
    64
}
fn default_length() -> u32 {
    600
}
