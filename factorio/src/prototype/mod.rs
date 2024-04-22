mod crafting_machine;
mod fluid;
mod item;
mod recipe;
mod resource;

use api::{CraftingMachinePrototype, FactorioType, Prototype};
pub use recipe::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Add;
use std::str::FromStr;

use crate::prototype::crafting_machine::CraftingMachines;
pub use resource::*;

pub struct PrototypeStage {
    crafting_machines: CraftingMachines,
    recipes: Recipes,
    resources: Resources,
    prototypes: Vec<Prototype>,
}

pub struct InvalidPrototype;

#[macro_export]
macro_rules! map_by_name {
    ($e:expr) => {
        // TODO: put collect in here?
        $e.iter()
            .map(|value| (value.name.clone(), value.clone()))
            .collect()
    };
}

#[macro_export]
macro_rules! group_by_field {
    ($e:expr, $f:ident) => {
        $e.iter()
            .group_by(|value| value.$f.clone())
            .into_iter()
            .map(|(field, values)| (field, map_by_name!(&values.cloned().collect::<Vec<_>>())))
            .collect()
    };
}

// TODO: find better name and make prettier
// requires imports, not very hygienic!
// probably just a generic function at this point
#[macro_export]
macro_rules! group_by_field_in_set {
    ($e:expr, $f:ident, $s:expr, $t:ident) => {
        $s.iter()
            .map(|s_value| {
                let expr = $e
                    .iter()
                    .filter(|e_value| contains!(e_value.$f, s_value, $t))
                    .cloned()
                    .collect::<Vec<_>>();
                (s_value.clone(), map_by_name!(expr))
            })
            .collect()
    };
}

#[macro_export]
macro_rules! contains {
    ($e:expr, $v:expr, map) => {
        $e.contains_key($v)
    };
    ($e:expr, $v:expr, set) => {
        $e.contains($v)
    };
}

impl FromStr for PrototypeStage {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let prototypes = parse_prototypes(s)?;
        let crafting_machines = (&prototypes).into();
        let recipes = (&prototypes, &crafting_machines).into();
        let resources = (&prototypes).into();
        Ok(PrototypeStage {
            crafting_machines,
            recipes,
            resources,
            prototypes,
        })
    }
}

impl PrototypeStage {
    pub fn crafting_machines(&self) -> &CraftingMachines {
        &self.crafting_machines
    }

    pub fn recipes(&self) -> &Recipes {
        &self.recipes
    }

    pub fn resources(&self) -> &Resources {
        &self.resources
    }

    pub fn prototypes(&self) -> &Vec<Prototype> {
        &self.prototypes
    }
}

// https://lua-api.factorio.com/latest/tree.html
fn parse_prototypes(json: &str) -> Result<Vec<Prototype>, serde_json::Error> {
    let mut parsed_prototypes = vec![];
    let data = serde_json::from_str::<serde_json::Value>(json)?;
    for (tag, prototypes) in data.as_object().unwrap() {
        let tag = get_tag(tag);
        // TODO: those expressions are huge and take too long to deserialize
        if tag == "named_noise_expression" {
            continue;
        }
        for (name, prototype) in prototypes.as_object().unwrap() {
            let mut prototype = prototype.as_object().unwrap().clone();
            prototype.insert(
                "serde_tag".to_owned(),
                serde_json::Value::String(tag.clone()),
            );
            let prototype =
                serde_json::from_value::<Prototype>(serde_json::Value::Object(prototype))?;
            parsed_prototypes.push(prototype);
        }
    }
    Ok(parsed_prototypes)
}

fn get_tag(tag: &str) -> String {
    let add_prototype = match tag {
        "combat-robot-count" => return "combat_robot_count_achievement_prototype".to_owned(),
        "optimized-decorative" => return "decorative_prototype".to_owned(),
        "particle" => return "entity_particle_prototype".to_owned(),
        "unit-spawner" => return "enemy_spawner_prototype".to_owned(),
        "loader-1x1" => return "loader1x1_prototype".to_owned(),
        "loader" => return "loader1x2_prototype".to_owned(),
        "fire" => return "fire_flame_prototype".to_owned(),
        "stream" => return "fluid_stream_prototype".to_owned(),
        "highlight-box" => return "highlight_box_entity_prototype".to_owned(),
        "resource" => return "resource_entity_prototype".to_owned(),
        "smoke" => return "simple_smoke_prototype".to_owned(),
        "ammo" => return "ammo_item_prototype".to_owned(),
        "blueprint" => return "blueprint_item_prototype".to_owned(),
        "item-subgroup" => return "item_sub_group".to_owned(),
        "noise-expression" => return "named_noise_expression".to_owned(),
        "optimized-particle" => return "particle_prototype".to_owned(),
        "tutorial" => return "tutorial_definition".to_owned(),
        "tile-effect" => return "tile_effect_definition".to_owned(),
        "ambient-sound"
        | "map-gen-presets"
        | "map-settings"
        | "mouse-cursor"
        | "ammo-category"
        | "autoplace-control"
        | "damage-type"
        | "equipment-category"
        | "fuel-category"
        | "gui-style"
        | "item-group"
        | "module-category"
        | "noise-layer"
        | "recipe-category"
        | "resource-category"
        | "tips-and-tricks-item"
        | "utility-constants"
        | "utility-sounds"
        | "utility-sprites"
        | "tips-and-tricks-item-category"
        | "trigger-target-type"
        | "wind-sound" => false,
        _ => true,
    };
    let tag = tag.replace("-", "_");
    if add_prototype {
        tag.add("_prototype")
    } else {
        tag
    }
}
