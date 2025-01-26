#[derive(Debug, serde::Deserialize)]
pub struct TileEffectDefinition {
    name: String,
    puddle: Option<crate::types::PuddleTileEffectParameters>,
    shader: TileEffectDefinitionShader,
    space: Option<crate::types::SpaceTileEffectParameters>,
    #[serde(rename = "type")]
    type_: String,
    water: Option<crate::types::WaterTileEffectParameters>,
}
#[derive(Debug, serde::Deserialize)]
pub enum TileEffectDefinitionShader {
    #[serde(rename = "water")]
    Water,
    #[serde(rename = "space")]
    Space,
    #[serde(rename = "puddle")]
    Puddle,
}
