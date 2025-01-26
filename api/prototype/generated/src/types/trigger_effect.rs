#[derive(Debug, serde::Deserialize)]
pub enum TriggerEffect {
    #[serde(untagged)]
    TriggerEffectVariants(TriggerEffectVariants),
    #[serde(untagged)]
    VecTriggerEffectVariants(Vec<TriggerEffectVariants>),
}
#[derive(Debug, serde::Deserialize)]
pub enum TriggerEffectVariants {
    #[serde(untagged)]
    DamageTriggerEffectItem(Box<crate::types::DamageTriggerEffectItem>),
    #[serde(untagged)]
    CreateEntityTriggerEffectItem(Box<crate::types::CreateEntityTriggerEffectItem>),
    #[serde(untagged)]
    CreateExplosionTriggerEffectItem(Box<crate::types::CreateExplosionTriggerEffectItem>),
    #[serde(untagged)]
    CreateFireTriggerEffectItem(Box<crate::types::CreateFireTriggerEffectItem>),
    #[serde(untagged)]
    CreateSmokeTriggerEffectItem(Box<crate::types::CreateSmokeTriggerEffectItem>),
    #[serde(untagged)]
    CreateTrivialSmokeEffectItem(Box<crate::types::CreateTrivialSmokeEffectItem>),
    #[serde(untagged)]
    CreateAsteroidChunkEffectItem(Box<crate::types::CreateAsteroidChunkEffectItem>),
    #[serde(untagged)]
    CreateParticleTriggerEffectItem(Box<crate::types::CreateParticleTriggerEffectItem>),
    #[serde(untagged)]
    CreateStickerTriggerEffectItem(Box<crate::types::CreateStickerTriggerEffectItem>),
    #[serde(untagged)]
    CreateDecorativesTriggerEffectItem(Box<crate::types::CreateDecorativesTriggerEffectItem>),
    #[serde(untagged)]
    NestedTriggerEffectItem(Box<crate::types::NestedTriggerEffectItem>),
    #[serde(untagged)]
    PlaySoundTriggerEffectItem(Box<crate::types::PlaySoundTriggerEffectItem>),
    #[serde(untagged)]
    PushBackTriggerEffectItem(Box<crate::types::PushBackTriggerEffectItem>),
    #[serde(untagged)]
    DestroyCliffsTriggerEffectItem(Box<crate::types::DestroyCliffsTriggerEffectItem>),
    #[serde(untagged)]
    ShowExplosionOnChartTriggerEffectItem(Box<crate::types::ShowExplosionOnChartTriggerEffectItem>),
    #[serde(untagged)]
    InsertItemTriggerEffectItem(Box<crate::types::InsertItemTriggerEffectItem>),
    #[serde(untagged)]
    ScriptTriggerEffectItem(Box<crate::types::ScriptTriggerEffectItem>),
    #[serde(untagged)]
    SetTileTriggerEffectItem(Box<crate::types::SetTileTriggerEffectItem>),
    #[serde(untagged)]
    InvokeTileEffectTriggerEffectItem(Box<crate::types::InvokeTileEffectTriggerEffectItem>),
    #[serde(untagged)]
    DestroyDecorativesTriggerEffectItem(Box<crate::types::DestroyDecorativesTriggerEffectItem>),
    #[serde(untagged)]
    CameraEffectTriggerEffectItem(Box<crate::types::CameraEffectTriggerEffectItem>),
    #[serde(untagged)]
    ActivateImpactTriggerEffectItem(Box<crate::types::ActivateImpactTriggerEffectItem>),
}
