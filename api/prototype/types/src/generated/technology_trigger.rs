pub enum TechnologyTrigger {
    MineEntityTechnologyTrigger(Box<MineEntityTechnologyTrigger>),
    CraftItemTechnologyTrigger(Box<CraftItemTechnologyTrigger>),
    CraftFluidTechnologyTrigger(Box<CraftFluidTechnologyTrigger>),
    SendItemToOrbitTechnologyTrigger(Box<SendItemToOrbitTechnologyTrigger>),
    CaptureSpawnerTechnologyTrigger(Box<CaptureSpawnerTechnologyTrigger>),
    BuildEntityTechnologyTrigger(Box<BuildEntityTechnologyTrigger>),
    CreateSpacePlatformTechnologyTrigger(Box<CreateSpacePlatformTechnologyTrigger>),
}
