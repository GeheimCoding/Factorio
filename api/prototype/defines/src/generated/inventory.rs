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
            1 => {
                use std::collections::HashSet;
                let mut variants = vec![];
                variants.push(Value1::ArtilleryTurretAmmo);
                variants.push(Value1::ArtilleryWagonAmmo);
                variants.push(Value1::BeaconModules);
                variants.push(Value1::CargoLandingPadMain);
                variants.push(Value1::CargoUnit);
                variants.push(Value1::CargoWagon);
                variants.push(Value1::CharacterCorpse);
                variants.push(Value1::CharacterMain);
                variants.push(Value1::Chest);
                variants.push(Value1::EditorMain);
                variants.push(Value1::Fuel);
                variants.push(Value1::HubMain);
                variants.push(Value1::ItemMain);
                variants.push(Value1::RoboportRobot);
                variants.push(Value1::RobotCargo);
                variants.push(Value1::TurretAmmo);
                Ok(Inventory::Value1(HashSet::from_iter(variants)))
            }
            11 => Ok(Inventory::RocketSiloTrash),
            2 => {
                use std::collections::HashSet;
                let mut variants = vec![];
                variants.push(Value2::AssemblingMachineInput);
                variants.push(Value2::CarTrunk);
                variants.push(Value2::CargoLandingPadTrash);
                variants.push(Value2::FurnaceSource);
                variants.push(Value2::GodMain);
                variants.push(Value2::HubTrash);
                variants.push(Value2::LabInput);
                variants.push(Value2::LogisticContainerTrash);
                variants.push(Value2::MiningDrillModules);
                variants.push(Value2::RoboportMaterial);
                variants.push(Value2::RobotRepair);
                variants.push(Value2::RocketSiloInput);
                variants.push(Value2::SpiderTrunk);
                Ok(Inventory::Value2(HashSet::from_iter(variants)))
            }
            3 => {
                use std::collections::HashSet;
                let mut variants = vec![];
                variants.push(Value3::AssemblingMachineOutput);
                variants.push(Value3::CarAmmo);
                variants.push(Value3::CharacterGuns);
                variants.push(Value3::EditorGuns);
                variants.push(Value3::FurnaceResult);
                variants.push(Value3::LabModules);
                variants.push(Value3::RocketSiloOutput);
                variants.push(Value3::SpiderAmmo);
                Ok(Inventory::Value3(HashSet::from_iter(variants)))
            }
            4 => {
                use std::collections::HashSet;
                let mut variants = vec![];
                variants.push(Value4::AssemblingMachineModules);
                variants.push(Value4::CharacterAmmo);
                variants.push(Value4::EditorAmmo);
                variants.push(Value4::FurnaceModules);
                variants.push(Value4::RocketSiloModules);
                variants.push(Value4::SpiderTrash);
                Ok(Inventory::Value4(HashSet::from_iter(variants)))
            }
            5 => {
                use std::collections::HashSet;
                let mut variants = vec![];
                variants.push(Value5::CharacterArmor);
                variants.push(Value5::EditorArmor);
                Ok(Inventory::Value5(HashSet::from_iter(variants)))
            }
            6 => Ok(Inventory::BurntResult),
            7 => {
                use std::collections::HashSet;
                let mut variants = vec![];
                variants.push(Value7::AssemblingMachineDump);
                variants.push(Value7::CharacterVehicle);
                Ok(Inventory::Value7(HashSet::from_iter(variants)))
            }
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
