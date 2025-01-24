#[derive(serde::Deserialize)]
pub struct PuddleTileEffectParameters {
    puddle_noise_texture: crate::types::EffectTexture,
    water_effect: Option<crate::types::TileEffectDefinitionID>,
    water_effect_parameters: Option<crate::types::WaterTileEffectParameters>,
}
