#[derive(serde::Deserialize)]
pub struct Effect {
    consumption: crate::types::EffectValue,
    pollution: crate::types::EffectValue,
    productivity: crate::types::EffectValue,
    quality: crate::types::EffectValue,
    speed: crate::types::EffectValue,
}
