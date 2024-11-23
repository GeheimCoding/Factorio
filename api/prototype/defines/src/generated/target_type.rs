#[derive(Debug, Clone, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum TargetType {
    Commandable = 16,
    CustomChartTag = 17,
    Entity = 1,
    Equipment = 2,
    EquipmentGrid = 3,
    GuiElement = 18,
    Item = 4,
    LogisticCell = 5,
    LogisticNetwork = 6,
    LogisticSection = 7,
    PermissionGroup = 8,
    Planet = 9,
    Player = 10,
    RailPath = 11,
    RenderObject = 12,
    SpacePlatform = 13,
    Surface = 14,
    Train = 15,
}
