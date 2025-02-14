#[derive(Debug, serde::Deserialize)]
pub struct EffectTexture {
    #[serde(flatten)]
    base_: crate::types::SpriteSource,
}
