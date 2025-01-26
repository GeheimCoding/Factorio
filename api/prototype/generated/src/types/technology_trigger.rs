#[derive(Debug, serde::Deserialize)]
pub enum TechnologyTrigger {
    #[serde(untagged)]
    MineEntityTechnologyTrigger(Box<crate::types::MineEntityTechnologyTrigger>),
    #[serde(untagged)]
    CraftItemTechnologyTrigger(Box<crate::types::CraftItemTechnologyTrigger>),
    #[serde(untagged)]
    CraftFluidTechnologyTrigger(Box<crate::types::CraftFluidTechnologyTrigger>),
    #[serde(untagged)]
    SendItemToOrbitTechnologyTrigger(Box<crate::types::SendItemToOrbitTechnologyTrigger>),
    #[serde(untagged)]
    CaptureSpawnerTechnologyTrigger(Box<crate::types::CaptureSpawnerTechnologyTrigger>),
    #[serde(untagged)]
    BuildEntityTechnologyTrigger(Box<crate::types::BuildEntityTechnologyTrigger>),
    #[serde(untagged)]
    CreateSpacePlatformTechnologyTrigger(Box<crate::types::CreateSpacePlatformTechnologyTrigger>),
}
