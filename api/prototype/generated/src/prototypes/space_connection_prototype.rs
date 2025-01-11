pub struct SpaceConnectionPrototype {
    asteroid_spawn_definitions: Vec<crate::types::SpaceConnectionAsteroidSpawnDefinition>,
    from: crate::types::SpaceLocationID,
    icon: crate::types::FileName,
    icon_size: crate::types::SpriteSizeType,
    icons: Vec<crate::types::IconData>,
    length: u32,
    to: crate::types::SpaceLocationID,
}
