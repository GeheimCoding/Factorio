#[derive(Debug, serde::Deserialize)]
pub struct Effect {
    consumption: Option<crate::types::EffectValue>,
    pollution: Option<crate::types::EffectValue>,
    productivity: Option<crate::types::EffectValue>,
    quality: Option<crate::types::EffectValue>,
    speed: Option<crate::types::EffectValue>,
}
