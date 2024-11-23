#[derive(Eq, Hash, PartialEq)]
pub enum Value1 {
    ArtilleryTurretAmmo,
    ArtilleryWagonAmmo,
    BeaconModules,
    CargoLandingPadMain,
    CargoUnit,
    CargoWagon,
    CharacterCorpse,
    CharacterMain,
    Chest,
    EditorMain,
    Fuel,
    HubMain,
    ItemMain,
    RoboportRobot,
    RobotCargo,
    TurretAmmo,
}
#[derive(Eq, Hash, PartialEq)]
pub enum Value2 {
    AssemblingMachineInput,
    CarTrunk,
    CargoLandingPadTrash,
    FurnaceSource,
    GodMain,
    HubTrash,
    LabInput,
    LogisticContainerTrash,
    MiningDrillModules,
    RoboportMaterial,
    RobotRepair,
    RocketSiloInput,
    SpiderTrunk,
}
#[derive(Eq, Hash, PartialEq)]
pub enum Value3 {
    AssemblingMachineOutput,
    CarAmmo,
    CharacterGuns,
    EditorGuns,
    FurnaceResult,
    LabModules,
    RocketSiloOutput,
    SpiderAmmo,
}
#[derive(Eq, Hash, PartialEq)]
pub enum Value4 {
    AssemblingMachineModules,
    CharacterAmmo,
    EditorAmmo,
    FurnaceModules,
    RocketSiloModules,
    SpiderTrash,
}
#[derive(Eq, Hash, PartialEq)]
pub enum Value5 {
    CharacterArmor,
    EditorArmor,
}
#[derive(Eq, Hash, PartialEq)]
pub enum Value7 {
    AssemblingMachineDump,
    CharacterVehicle,
}
impl<'de> serde::Deserialize<'de> for Inventory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match <u16 as serde::Deserialize>::deserialize(deserializer)? {
            1 => Ok(Inventory::Value1(std::collections::HashSet::from([
                Value1::ArtilleryTurretAmmo,
                Value1::ArtilleryWagonAmmo,
                Value1::BeaconModules,
                Value1::CargoLandingPadMain,
                Value1::CargoUnit,
                Value1::CargoWagon,
                Value1::CharacterCorpse,
                Value1::CharacterMain,
                Value1::Chest,
                Value1::EditorMain,
                Value1::Fuel,
                Value1::HubMain,
                Value1::ItemMain,
                Value1::RoboportRobot,
                Value1::RobotCargo,
                Value1::TurretAmmo,
            ]))),
            11 => Ok(Inventory::RocketSiloTrash),
            2 => Ok(Inventory::Value2(std::collections::HashSet::from([
                Value2::AssemblingMachineInput,
                Value2::CarTrunk,
                Value2::CargoLandingPadTrash,
                Value2::FurnaceSource,
                Value2::GodMain,
                Value2::HubTrash,
                Value2::LabInput,
                Value2::LogisticContainerTrash,
                Value2::MiningDrillModules,
                Value2::RoboportMaterial,
                Value2::RobotRepair,
                Value2::RocketSiloInput,
                Value2::SpiderTrunk,
            ]))),
            3 => Ok(Inventory::Value3(std::collections::HashSet::from([
                Value3::AssemblingMachineOutput,
                Value3::CarAmmo,
                Value3::CharacterGuns,
                Value3::EditorGuns,
                Value3::FurnaceResult,
                Value3::LabModules,
                Value3::RocketSiloOutput,
                Value3::SpiderAmmo,
            ]))),
            4 => Ok(Inventory::Value4(std::collections::HashSet::from([
                Value4::AssemblingMachineModules,
                Value4::CharacterAmmo,
                Value4::EditorAmmo,
                Value4::FurnaceModules,
                Value4::RocketSiloModules,
                Value4::SpiderTrash,
            ]))),
            5 => Ok(Inventory::Value5(std::collections::HashSet::from([
                Value5::CharacterArmor,
                Value5::EditorArmor,
            ]))),
            6 => Ok(Inventory::BurntResult),
            7 => Ok(Inventory::Value7(std::collections::HashSet::from([
                Value7::AssemblingMachineDump,
                Value7::CharacterVehicle,
            ]))),
            8 => Ok(Inventory::CharacterTrash),
            9 => Ok(Inventory::RocketSiloRocket),
            other => Err(serde::de::Error::custom(format!(
                "unexpected value: {other}"
            ))),
        }
    }
}
pub enum Inventory {
    Value1(std::collections::HashSet<Value1>),
    RocketSiloTrash,
    Value2(std::collections::HashSet<Value2>),
    Value3(std::collections::HashSet<Value3>),
    Value4(std::collections::HashSet<Value4>),
    Value5(std::collections::HashSet<Value5>),
    BurntResult,
    Value7(std::collections::HashSet<Value7>),
    CharacterTrash,
    RocketSiloRocket,
}
