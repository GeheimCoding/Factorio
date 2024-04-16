use api::{
    AssemblingMachinePrototype, CraftingMachinePrototype, FurnacePrototype, Prototype,
    RocketSiloPrototype,
};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct AssemblingMachineData {
    pub ingredient_count: u8,
}

#[derive(Debug)]
pub struct FurnaceData {
    pub result_inventory_size: u16,
    pub source_inventory_size: u16,
}

#[derive(Debug)]
pub struct RocketSiloData {
    pub parent: AssemblingMachineData,
    // TODO: extend
}

#[derive(Debug)]
pub enum CraftingMachineVariant {
    AssemblingMachine(AssemblingMachineData),
    Furnace(FurnaceData),
    RocketSilo(RocketSiloData),
}

#[derive(Debug)]
pub struct CraftingMachine {
    pub name: String,
    pub categories: HashSet<String>,
    pub speed: f64,
    pub variant: CraftingMachineVariant,
}

pub type CraftingMachinesByName = HashMap<String, CraftingMachine>;

#[derive(Debug)]
pub struct CraftingMachines {
    pub by_name: CraftingMachinesByName,
    pub by_category: HashSet<String, CraftingMachinesByName>,
}

impl From<&AssemblingMachinePrototype> for AssemblingMachineData {
    fn from(assembling_machine: &AssemblingMachinePrototype) -> Self {
        Self {
            ingredient_count: assembling_machine.ingredient_count.unwrap_or(u8::MAX),
        }
    }
}

impl From<&AssemblingMachinePrototype> for CraftingMachineVariant {
    fn from(assembling_machine: &AssemblingMachinePrototype) -> Self {
        Self::AssemblingMachine(assembling_machine.into())
    }
}

impl From<&FurnacePrototype> for FurnaceData {
    fn from(furnace: &FurnacePrototype) -> Self {
        Self {
            result_inventory_size: furnace.result_inventory_size,
            source_inventory_size: furnace.source_inventory_size,
        }
    }
}

// TODO: split into one file per struct?
impl From<&FurnacePrototype> for CraftingMachineVariant {
    fn from(furnace: &FurnacePrototype) -> Self {
        Self::Furnace(furnace.into())
    }
}

impl From<&RocketSiloPrototype> for RocketSiloData {
    fn from(rocket_silo: &RocketSiloPrototype) -> Self {
        Self {
            parent: (&rocket_silo.parent_).into(),
        }
    }
}

impl From<&RocketSiloPrototype> for CraftingMachineVariant {
    fn from(rocket_silo: &RocketSiloPrototype) -> Self {
        Self::RocketSilo(rocket_silo.into())
    }
}

impl From<(&CraftingMachinePrototype, CraftingMachineVariant)> for CraftingMachine {
    fn from(
        (crafting_machine, variant): (&CraftingMachinePrototype, CraftingMachineVariant),
    ) -> Self {
        Self {
            name: crafting_machine
                .parent_
                .parent_
                .parent_
                .parent_
                .name
                .clone(),
            categories: HashSet::from_iter(crafting_machine.crafting_categories.iter().cloned()),
            speed: *crafting_machine.crafting_speed.as_value().unwrap(),
            variant,
        }
    }
}

impl TryFrom<&Prototype> for CraftingMachine {
    // TODO: proper error handling
    type Error = ();

    fn try_from(prototype: &Prototype) -> Result<Self, Self::Error> {
        let (crafting_machine, variant) =
            if let Some(assembling_machine) = prototype.as_assembling_machine_prototype() {
                (&assembling_machine.parent_, assembling_machine.into())
            } else if let Some(furnace) = prototype.as_furnace_prototype() {
                (&furnace.parent_, furnace.into())
            } else if let Some(rocket_silo) = prototype.as_rocket_silo_prototype() {
                (&rocket_silo.parent_.parent_, rocket_silo.into())
            } else {
                return Err(());
            };
        Ok((crafting_machine, variant).into())
    }
}
