pub enum Type {
    Accumulator,
    AgriculturalTower,
    ArithmeticCombinator,
    ArtilleryTurret,
    AssemblingMachine,
    AsteroidCollector,
    CargoLandingPad,
    ConstantCombinator,
    Container,
    DeciderCombinator,
    DisplayPanel,
    GenericOnOff,
    Inserter,
    Lamp,
    Loader,
    LogisticContainer,
    MiningDrill,
    ProgrammableSpeaker,
    Pump,
    Radar,
    RailChainSignal,
    RailSignal,
    Reactor,
    Roboport,
    RocketSilo,
    SelectorCombinator,
    SpacePlatformHub,
    StorageTank,
    TrainStop,
    TransportBelt,
    Turret,
    Wall,
}
pub enum ContentReadMode {
    EntireBeltHold,
    Hold,
    Pulse,
}
pub enum TransportBelt {
    ContentReadMode(ContentReadMode),
}
pub enum ReadMode {
    LogisticInventory,
    None,
    OrbitalRequests,
}
pub enum RocketSilo {
    ReadMode(ReadMode),
}
pub enum ReadItemsMode {
    Logistics,
    MissingRequests,
    None,
}
pub enum Roboport {
    ReadItemsMode(ReadItemsMode),
}
pub enum ResourceReadMode {
    EntirePatch,
    ThisMiner,
}
pub enum MiningDrill {
    ResourceReadMode(ResourceReadMode),
}
pub enum ExclusiveMode {
    None,
    SendContents,
    SetRequests,
}
pub enum LogisticContainer {
    ExclusiveMode(ExclusiveMode),
}
pub enum ColorMode {
    ColorMapping,
    Components,
    PackedRgb,
}
pub enum Lamp {
    ColorMode(ColorMode),
}
pub enum HandReadMode {
    Hold,
    Pulse,
}
pub enum Inserter {
    HandReadMode(HandReadMode),
}
pub enum CargoLandingPad {
    ExclusiveMode(ExclusiveMode),
}
pub enum ControlBehavior {
    CargoLandingPad(CargoLandingPad),
    Inserter(Inserter),
    Lamp(Lamp),
    LogisticContainer(LogisticContainer),
    MiningDrill(MiningDrill),
    Roboport(Roboport),
    RocketSilo(RocketSilo),
    TransportBelt(TransportBelt),
    Type(Type),
}
