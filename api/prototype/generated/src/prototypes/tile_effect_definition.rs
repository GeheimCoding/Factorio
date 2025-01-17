pub struct TileEffectDefinition {
    name: String,
    puddle: crate::types::PuddleTileEffectParameters,
    shader: TileEffectDefinitionShader,
    space: crate::types::SpaceTileEffectParameters,
    type_: String,
    water: crate::types::WaterTileEffectParameters,
}
#[derive(serde::Deserialize)]
pub enum TileEffectDefinitionShader {
    #[serde(rename = "water")]
    Water,
    #[serde(rename = "space")]
    Space,
    #[serde(rename = "puddle")]
    Puddle,
}
