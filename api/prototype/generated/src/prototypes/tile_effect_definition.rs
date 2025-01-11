pub struct TileEffectDefinition {
    name: String,
    puddle: crate::types::PuddleTileEffectParameters,
    shader: TileEffectDefinitionShader,
    space: crate::types::SpaceTileEffectParameters,
    type_: String,
    water: crate::types::WaterTileEffectParameters,
}
pub enum TileEffectDefinitionShader {
    Water,
    Space,
    Puddle,
}
