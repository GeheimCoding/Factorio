#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub enum CargoLandingPad {
    ExclusiveMode(ExclusiveMode),
}
#[derive(Debug, Clone)]
pub enum Inserter {
    HandReadMode(HandReadMode),
}
#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum HandReadMode {
    Hold = 1,
    Pulse = 0,
}
#[derive(Debug, Clone)]
pub enum Lamp {
    ColorMode(ColorMode),
}
#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum ColorMode {
    ColorMapping = 0,
    Components = 1,
    PackedRgb = 2,
}
#[derive(Debug, Clone)]
pub enum LogisticContainer {
    ExclusiveMode(ExclusiveMode),
}
#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum ExclusiveMode {
    None = 2,
    SendContents = 0,
    SetRequests = 1,
}
#[derive(Debug, Clone)]
pub enum MiningDrill {
    ResourceReadMode(ResourceReadMode),
}
#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum ResourceReadMode {
    EntirePatch = 1,
    ThisMiner = 0,
}
#[derive(Debug, Clone)]
pub enum Roboport {
    ReadItemsMode(ReadItemsMode),
}
#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum ReadItemsMode {
    Logistics = 1,
    MissingRequests = 2,
    None = 0,
}
#[derive(Debug, Clone)]
pub enum RocketSilo {
    ReadMode(ReadMode),
}
#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum ReadMode {
    LogisticInventory = 1,
    None = 0,
    OrbitalRequests = 2,
}
#[derive(Debug, Clone)]
pub enum TransportBelt {
    ContentReadMode(ContentReadMode),
}
#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum ContentReadMode {
    EntireBeltHold = 2,
    Hold = 1,
    Pulse = 0,
}
#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum Type {
    Accumulator = 13,
    AgriculturalTower = 32,
    ArithmeticCombinator = 10,
    ArtilleryTurret = 26,
    AssemblingMachine = 19,
    AsteroidCollector = 28,
    CargoLandingPad = 31,
    ConstantCombinator = 11,
    Container = 1,
    DeciderCombinator = 9,
    DisplayPanel = 29,
    GenericOnOff = 2,
    Inserter = 3,
    Lamp = 4,
    Loader = 30,
    LogisticContainer = 5,
    MiningDrill = 16,
    ProgrammableSpeaker = 17,
    Pump = 21,
    Radar = 27,
    RailChainSignal = 18,
    RailSignal = 14,
    Reactor = 24,
    Roboport = 6,
    RocketSilo = 22,
    SelectorCombinator = 20,
    SpacePlatformHub = 25,
    StorageTank = 7,
    TrainStop = 8,
    TransportBelt = 12,
    Turret = 23,
    Wall = 15,
}
