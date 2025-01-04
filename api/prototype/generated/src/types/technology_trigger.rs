pub enum TechnologyTrigger {
    MineEntityTechnologyTrigger(Box<crate::types::MineEntityTechnologyTrigger>),
    CraftItemTechnologyTrigger(Box<crate::types::CraftItemTechnologyTrigger>),
    CraftFluidTechnologyTrigger(Box<crate::types::CraftFluidTechnologyTrigger>),
    SendItemToOrbitTechnologyTrigger(Box<crate::types::SendItemToOrbitTechnologyTrigger>),
    CaptureSpawnerTechnologyTrigger(Box<crate::types::CaptureSpawnerTechnologyTrigger>),
    BuildEntityTechnologyTrigger(Box<crate::types::BuildEntityTechnologyTrigger>),
    CreateSpacePlatformTechnologyTrigger(Box<crate::types::CreateSpacePlatformTechnologyTrigger>),
}
