#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RelativeGuiType {
    AccumulatorGui,
    AchievementGui,
    AdditionalEntityInfoGui,
    AdminGui,
    AgricultureTowerGui,
    ArithmeticCombinatorGui,
    ArmorGui,
    AssemblingMachineGui,
    AssemblingMachineSelectRecipeGui,
    AsteroidCollectorGui,
    BeaconGui,
    BlueprintBookGui,
    BlueprintLibraryGui,
    BlueprintSetupGui,
    BonusGui,
    BurnerEquipmentGui,
    CarGui,
    CargoLandingPadGui,
    ConstantCombinatorGui,
    ContainerGui,
    ControllerGui,
    DeciderCombinatorGui,
    DeconstructionItemGui,
    DisplayPanelGui,
    ElectricEnergyInterfaceGui,
    ElectricNetworkGui,
    EntityVariationsGui,
    EntityWithEnergySourceGui,
    EquipmentGridGui,
    FurnaceGui,
    GenericOnOffEntityGui,
    GhostPickerGui,
    GlobalElectricNetworkGui,
    HeatInterfaceGui,
    InfinityPipeGui,
    InserterGui,
    ItemWithInventoryGui,
    LabGui,
    LampGui,
    LinkedContainerGui,
    LoaderGui,
    LogisticGui,
    MarketGui,
    MiningDrillGui,
    OtherPlayerGui,
    PermissionsGui,
    PickStopGui,
    PipeGui,
    PowerSwitchGui,
    ProductionGui,
    ProgrammableSpeakerGui,
    PumpGui,
    RailSignalBaseGui,
    ReactorGui,
    ResourceEntityGui,
    RoboportGui,
    RocketSiloGui,
    ScriptInventoryGui,
    SelectorCombinatorGui,
    ServerConfigGui,
    SpacePlatformHubGui,
    SpiderVehicleGui,
    SplitterGui,
    StandaloneCharacterGui,
    StorageTankGui,
    TileVariationsGui,
    TipsAndTricksGui,
    TrainGui,
    TrainStopGui,
    TrainsGui,
    TransportBeltGui,
    TurretGui,
    UpgradeItemGui,
    WallGui,
}
