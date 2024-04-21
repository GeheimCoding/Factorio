#![allow(unused)]
#![deny(clippy::unwrap_used)]

use std::any::{Any, TypeId};
use std::collections::HashSet;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::ops::Add;
use std::rc::Rc;
use std::str::FromStr;
use std::{collections::HashMap, fs, io, path::PathBuf, thread::sleep, time::Duration};

use anyhow::{Context, Result};
use serde_json::Number;
use zip::ZipArchive;

use crate::prototype::{map_recipe, PrototypeStage, Recipe, Resources};
use crate::runtime::{LuaObjects, RuntimeStage};
use api::{
    parse_factorio_type, Animation4Way, FactorioType, GunPrototype, ItemIngredientPrototype,
    LayeredSound, LuaAISettings, LuaAchievementPrototype, LuaGameScript, LuaGroup, LuaRecipe,
    LuaRecipePrototype, MaybeCycle, ProjectileAttackParameters, RecipeData, RecipePrototype,
    RecipePrototypeExpensive, TechnologyDataMaxLevel,
};
use extensions::{LuaObject, Traversable};
use remote_console::RemoteConsole;

mod prototype;
mod runtime;

// TODO: try to split into more libs to decrease build time
// TODO: add flattened map to FactorioType and warn if this has some entries after deserialization?
fn main() -> Result<()> {
    let prototype_stage = parse_prototype_stage()?;
    let recipes = parse_recipes_by_category(prototype_stage.recipes());

    let resources: Resources = prototype_stage.prototypes().into();
    println!("{resources:?}");

    // let mut runtime_stage = RuntimeStage::new();
    // let game = fs::read_to_string("output/game.json")?;
    // runtime_stage.add_factorio_type(&game)?;

    // remote_console()?;
    Ok(())
}

fn parse_prototype_stage() -> Result<PrototypeStage> {
    let mut archive = ZipArchive::new(File::open("api/json/data-raw-dump-v1.1.101.zip")?)?;
    let mut file = ZipArchive::by_name(&mut archive, "data-raw-dump.json")?;
    let mut data = String::new();

    file.read_to_string(&mut data)?;
    Ok(PrototypeStage::from_str(&data)?)
}

fn parse_recipes_by_category(
    recipes: &HashMap<String, Recipe>,
) -> HashMap<String, HashMap<String, Recipe>> {
    let mut recipes_by_category = recipes
        .iter()
        .map(|(_, recipe)| (recipe.category.clone(), HashMap::new()))
        .collect::<HashMap<_, _>>();
    recipes.iter().for_each(|(name, recipe)| {
        recipes_by_category
            .get_mut(&recipe.category)
            .unwrap()
            // TODO: remove clone / move to prototype stage?
            .insert(name.clone(), recipe.clone());
    });
    recipes_by_category
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
        // listen_to_events(&mut console)?;
        println!("{response}");
        // let game = parse_factorio_type(&response)?;
        // println!("{game:#?}");
    }
    Ok(())
}

fn listen_to_events(console: &mut RemoteConsole, runtime_stage: &mut RuntimeStage) -> Result<()> {
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
                let result = runtime_stage.add_factorio_type(event);
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
