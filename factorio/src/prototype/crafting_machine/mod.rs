mod assembling_machine;
mod furnace;
mod rocket_silo;

use crate::prototype::crafting_machine::assembling_machine::AssemblingMachineData;
use crate::prototype::crafting_machine::furnace::FurnaceData;
use crate::prototype::crafting_machine::rocket_silo::RocketSiloData;
use crate::prototype::InvalidPrototype;
use crate::{group_by_field, map_by_name};
use api::{
    AssemblingMachinePrototype, CraftingMachinePrototype, FurnacePrototype, Prototype,
    RocketSiloPrototype,
};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub enum CraftingMachineVariant {
    AssemblingMachine(AssemblingMachineData),
    Furnace(FurnaceData),
    RocketSilo(RocketSiloData),
}

#[derive(Clone, Debug)]
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
    pub by_category: HashMap<String, CraftingMachinesByName>,
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
    type Error = InvalidPrototype;

    fn try_from(prototype: &Prototype) -> Result<Self, Self::Error> {
        use CraftingMachineVariant::*;
        Ok(
            if let Some(assembling_machine) = prototype.as_assembling_machine_prototype() {
                (
                    &assembling_machine.parent_,
                    AssemblingMachine(assembling_machine.into()),
                )
            } else if let Some(furnace) = prototype.as_furnace_prototype() {
                (&furnace.parent_, Furnace(furnace.into()))
            } else if let Some(rocket_silo) = prototype.as_rocket_silo_prototype() {
                (&rocket_silo.parent_.parent_, RocketSilo(rocket_silo.into()))
            } else {
                return Err(InvalidPrototype);
            }
            .into(),
        )
    }
}

impl From<&Vec<Prototype>> for CraftingMachines {
    fn from(prototypes: &Vec<Prototype>) -> Self {
        let crafting_machines = prototypes
            .iter()
            .filter_map(|prototype| prototype.try_into().ok())
            .collect::<Vec<CraftingMachine>>();
        let categories = crafting_machines
            .iter()
            .flat_map(|crafting_machine| crafting_machine.categories.clone())
            .collect::<HashSet<_>>();
        Self {
            by_name: map_by_name!(crafting_machines),
            by_category: categories
                .iter()
                .map(|category| {
                    (
                        category.clone(),
                        map_by_name!(crafting_machines
                            .iter()
                            .filter(|crafting_machine| {
                                crafting_machine.categories.contains(category)
                            })
                            .cloned()
                            .collect::<Vec<_>>()),
                    )
                })
                .collect(),
        }
    }
}
