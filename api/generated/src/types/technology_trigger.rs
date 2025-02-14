#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type")]
pub enum TechnologyTrigger {
    #[serde(rename = "mine-entity")]
    MineEntityTechnologyTrigger(Box<crate::types::MineEntityTechnologyTrigger>),
    #[serde(rename = "craft-item")]
    CraftItemTechnologyTrigger(Box<crate::types::CraftItemTechnologyTrigger>),
    #[serde(rename = "craft-fluid")]
    CraftFluidTechnologyTrigger(Box<crate::types::CraftFluidTechnologyTrigger>),
    #[serde(rename = "send-item-to-orbit")]
    SendItemToOrbitTechnologyTrigger(Box<crate::types::SendItemToOrbitTechnologyTrigger>),
    #[serde(rename = "capture-spawner")]
    CaptureSpawnerTechnologyTrigger(Box<crate::types::CaptureSpawnerTechnologyTrigger>),
    #[serde(rename = "build-entity")]
    BuildEntityTechnologyTrigger(Box<crate::types::BuildEntityTechnologyTrigger>),
    #[serde(rename = "create-space-platform")]
    CreateSpacePlatformTechnologyTrigger(Box<crate::types::CreateSpacePlatformTechnologyTrigger>),
}
