#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum GuiType {
    Achievement = 8,
    BlueprintLibrary = 9,
    Bonus = 6,
    Controller = 3,
    Custom = 16,
    Entity = 1,
    Equipment = 10,
    GlobalElectricNetwork = 22,
    Item = 5,
    Logistic = 11,
    None = 0,
    OpenedEntityGrid = 24,
    OtherPlayer = 12,
    Permissions = 14,
    PlayerManagement = 18,
    Production = 4,
    ScriptInventory = 23,
    ServerManagement = 17,
    Tile = 19,
    Trains = 7,
}
