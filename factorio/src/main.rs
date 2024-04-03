#![allow(unused)]
#![deny(clippy::unwrap_used)]

use std::any::{Any, TypeId};
use std::collections::HashSet;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::ops::Add;
use std::rc::Rc;
use std::{collections::HashMap, fs, io, path::PathBuf, thread::sleep, time::Duration};

use anyhow::{Context, Result};
use serde_json::Number;
use zip::ZipArchive;

use crate::recipe::{map_recipe, Ingredient, Product, Recipe, Type};
use api::{
    parse_factorio_type, Animation4Way, FactorioType, GunPrototype, ItemIngredientPrototype,
    LayeredSound, LuaAISettings, LuaAchievementPrototype, LuaGameScript, LuaGroup, LuaObjects,
    LuaRecipe, LuaRecipePrototype, MaybeCycle, ProjectileAttackParameters, Prototype, RecipeData,
    RecipePrototypeExpensive, Runtime, TechnologyDataMaxLevel,
};
use extensions::{LuaObject, Traversable};
use remote_console::RemoteConsole;

mod recipe;

fn main() -> Result<()> {
    let mut runtime = Runtime::new();

    // TODO: parse recipes from prototypes and compare against game prototypes
    let prototypes = parse_prototypes()?;
    println!("parsed {} prototypes", prototypes.len());

    // remote_console()?;
    // parse_recipes(&mut lua_objects).context("parse_recipes")?;

    // let content = fs::read_to_string("output/game.json")?;
    // parse_factorio_type(&content, &mut runtime)?;
    // let recipes = parse_recipes_by_category(&runtime).context("recipes")?;
    // println!("{recipes:#?}");

    // use_lua_object("65", runtime.lua_objects()).context("use_lua_object")?;
    // println!("{}", runtime.factorio_types().len());

    Ok(())
}

// TODO: add flattened map to FactorioType and warn if this has some entries after deserialization?
// TODO: move to lib?
// TODO: prevent doc string errors
// TODO: hide abstract classes from factorio_types
// https://lua-api.factorio.com/latest/tree.html
fn parse_prototypes() -> Result<Vec<Prototype>> {
    let mut archive = ZipArchive::new(File::open("api/json/data-raw-dump-v1.1.101.zip")?)?;
    let mut file = ZipArchive::by_name(&mut archive, "data-raw-dump.json")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut parsed_prototypes = vec![];
    let data = serde_json::from_str::<serde_json::Value>(&content)?;
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

fn parse_recipes_by_category(runtime: &Runtime) -> Option<HashMap<String, Vec<Recipe>>> {
    let game = runtime.factorio_types()[0]
        .as_class()?
        .as_lua_game_script()?;

    let mut recipes_by_category = game
        .recipe_category_prototypes
        .iter()
        .map(|(name, _)| (name.clone(), vec![]))
        .collect::<HashMap<_, _>>();

    for (name, recipe) in &game.recipe_prototypes {
        let recipe = recipe.resolve(runtime.lua_objects())?;
        recipes_by_category
            .get_mut(&recipe.category)?
            .push(map_recipe(recipe));
    }
    Some(recipes_by_category)
}

fn use_lua_object(class_id: &str, lua_objects: &LuaObjects) -> Option<()> {
    println!(
        "{}",
        lua_objects
            .get(class_id)?
            .as_any()
            .downcast_ref::<LuaGroup>()?
            .group
            .as_ref()?
            .resolve(&lua_objects)?
            .class_id
    );
    Some(())
}

fn parse_recipes(game: &LuaGameScript) -> Option<()> {
    let recipes = get_recipes(game)?;
    let mut recipes = recipes.iter().map(|(k, v)| k).cloned().collect::<Vec<_>>();
    recipes.sort();
    let recipe_prototypes = &game.recipe_prototypes;

    println!("{}", recipes.len());
    println!("{}", recipe_prototypes.len());
    println!("{}", recipes.join("\n"));

    Some(())
}

// https://wiki.factorio.com/Materials_and_recipes
fn get_recipes(game: &LuaGameScript) -> Option<&HashMap<String, MaybeCycle<LuaRecipe>>> {
    Some(
        &game
            .forces
            .get(&api::LuaGameScriptForces::String("player".to_owned()))?
            .as_value()?
            .recipes,
    )
}

// https://developer.valvesoftware.com/wiki/Source_RCON_Protocol
fn remote_console() -> Result<()> {
    let mut console = RemoteConsole::new("10.243.118.233", 25575, "123")?;
    let setup_response = setup(&mut console)?;
    if !setup_response.is_empty() {
        println!("{setup_response}");
    } else {
        let response = console.send_command(
            "
            rcon.print(Json.to_string(game))
        ",
        )?;
        //listen_to_events(&mut console)?;
        println!("{response}");
        //let game = parse_factorio_type(&response)?;
        //println!("{game:#?}");
    }
    Ok(())
}

fn listen_to_events(console: &mut RemoteConsole, runtime: &mut Runtime) -> Result<()> {
    let paths = fs::read_dir("output/events")?;
    let mut index = paths.count();
    loop {
        if index >= 1000 {
            break;
        }
        let response = console.send_command("poll_event_queue()")?;
        if !response.is_empty() {
            let events: Vec<_> = response.split("\n\n").collect();
            for event in events {
                let result = parse_factorio_type(event, runtime);
                if let Err(e) = result {
                    println!("{index}.json: {e}");
                    let filename = PathBuf::from(&format!("output/events/{index}.json"));
                    fs::write(filename, event)?;
                    index += 1;
                }
            }
        }
        sleep(Duration::from_millis(500));
    }
    Ok(())
}

fn setup(console: &mut RemoteConsole) -> io::Result<String> {
    let mut command = fs::read_to_string("lua/setup.lua")?;
    command.push('\n');
    command.push_str(&fs::read_to_string("lua/subclasses.lua")?);
    command.push('\n');
    command.push_str(&fs::read_to_string("lua/dictionaries.lua")?);
    command.push('\n');
    command.push_str(&fs::read_to_string("lua/settings.lua")?);
    command.push('\n');
    command.push_str(&fs::read_to_string("lua/manual_patches.lua")?);
    console.send_command(&command)
}
