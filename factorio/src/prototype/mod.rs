mod recipe;

use api::Prototype;
pub use recipe::*;
use std::ops::Add;
use std::str::FromStr;

pub struct PrototypeStage {
    recipes: Vec<Recipe>,
    prototypes: Vec<Prototype>,
}

impl FromStr for PrototypeStage {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let prototypes = parse_prototypes(s)?;
        Ok(PrototypeStage {
            recipes: vec![],
            prototypes,
        })
    }
}

impl PrototypeStage {
    pub fn recipes(&self) -> &Vec<Recipe> {
        &self.recipes
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
