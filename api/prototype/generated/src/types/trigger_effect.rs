#[derive(Debug, serde::Deserialize)]
pub enum TriggerEffect {
    #[serde(untagged)]
    TriggerEffectVariants(TriggerEffectVariants),
    #[serde(untagged)]
    VecTriggerEffectVariants(crate::vec::Vec<TriggerEffectVariants>),
}
#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type")]
pub enum TriggerEffectVariants {
    #[serde(rename = "damage")]
    DamageTriggerEffectItem(Box<crate::types::DamageTriggerEffectItem>),
    #[serde(rename = "create-entity")]
    CreateEntityTriggerEffectItem(Box<crate::types::CreateEntityTriggerEffectItem>),
    #[serde(rename = "create-explosion")]
    CreateExplosionTriggerEffectItem(Box<crate::types::CreateExplosionTriggerEffectItem>),
    #[serde(rename = "create-fire")]
    CreateFireTriggerEffectItem(Box<crate::types::CreateFireTriggerEffectItem>),
    #[serde(rename = "create-smoke")]
    CreateSmokeTriggerEffectItem(Box<crate::types::CreateSmokeTriggerEffectItem>),
    #[serde(rename = "create-trivial-smoke")]
    CreateTrivialSmokeEffectItem(Box<crate::types::CreateTrivialSmokeEffectItem>),
    #[serde(rename = "create-asteroid-chunk")]
    CreateAsteroidChunkEffectItem(Box<crate::types::CreateAsteroidChunkEffectItem>),
    #[serde(rename = "create-particle")]
    CreateParticleTriggerEffectItem(Box<crate::types::CreateParticleTriggerEffectItem>),
    #[serde(rename = "create-sticker")]
    CreateStickerTriggerEffectItem(Box<crate::types::CreateStickerTriggerEffectItem>),
    #[serde(rename = "create-decorative")]
    CreateDecorativesTriggerEffectItem(Box<crate::types::CreateDecorativesTriggerEffectItem>),
    #[serde(rename = "nested-result")]
    NestedTriggerEffectItem(Box<crate::types::NestedTriggerEffectItem>),
    #[serde(rename = "play-sound")]
    PlaySoundTriggerEffectItem(Box<crate::types::PlaySoundTriggerEffectItem>),
    #[serde(rename = "push-back")]
    PushBackTriggerEffectItem(Box<crate::types::PushBackTriggerEffectItem>),
    #[serde(rename = "destroy-cliffs")]
    DestroyCliffsTriggerEffectItem(Box<crate::types::DestroyCliffsTriggerEffectItem>),
    #[serde(rename = "show-explosion-on-chart")]
    ShowExplosionOnChartTriggerEffectItem(Box<crate::types::ShowExplosionOnChartTriggerEffectItem>),
    #[serde(rename = "insert-item")]
    InsertItemTriggerEffectItem(Box<crate::types::InsertItemTriggerEffectItem>),
    #[serde(rename = "script")]
    ScriptTriggerEffectItem(Box<crate::types::ScriptTriggerEffectItem>),
    #[serde(rename = "set-tile")]
    SetTileTriggerEffectItem(Box<crate::types::SetTileTriggerEffectItem>),
    #[serde(rename = "invoke-tile-trigger")]
    InvokeTileEffectTriggerEffectItem(Box<crate::types::InvokeTileEffectTriggerEffectItem>),
    #[serde(rename = "destroy-decoratives")]
    DestroyDecorativesTriggerEffectItem(Box<crate::types::DestroyDecorativesTriggerEffectItem>),
    #[serde(rename = "camera-effect")]
    CameraEffectTriggerEffectItem(Box<crate::types::CameraEffectTriggerEffectItem>),
    #[serde(rename = "activate-impact")]
    ActivateImpactTriggerEffectItem(Box<crate::types::ActivateImpactTriggerEffectItem>),
}
