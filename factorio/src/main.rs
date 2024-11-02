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
use std::{collections::HashMap, fs, io, path::PathBuf, thread, thread::sleep, time::Duration};

use anyhow::{Context, Result};
use enigo::Direction::{Click, Press, Release};
use enigo::{Button, Coordinate, Enigo, Key, Keyboard, Mouse, Settings};
use serde_json::Number;
use zip::ZipArchive;

use crate::prototype::{PrototypeStage, Recipe, Recipes, Resources};
use crate::runtime::{LuaObjects, RuntimeStage};
use api::Define::MouseButtonType;
use api::Type::Direction;
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
    // let prototype_stage = parse_prototype_stage()?;
    // println!("{:#?}", prototype_stage.crafting_machines());
    // println!("{:#?}", prototype_stage.recipes());
    // println!("{:#?}", prototype_stage.resources());
    // println!("{:#?}", prototype_stage.items());

    // process_recipes(&prototype_stage);
    // enigo()?;
    tiles();

    // let mut runtime_stage = RuntimeStage::new();
    // let game = fs::read_to_string("output/game.json")?;
    // runtime_stage.add_factorio_type(&game)?;

    // remote_console(&mut runtime_stage)?;
    Ok(())
}

struct ChunkPosition {
    pub x: i32,
    pub y: i32,
}

struct Chunks {
    pub in_queue: Vec<ChunkPosition>,
    pub current_row: u32,
    pub processed: Vec<ChunkPosition>,
    pub tiles: HashMap<(i32, i32), String>,
}

fn tiles() {
    /* TODO: create in_queue list of "to be converted chunks"
        initial: LuaSurface::get_chunks -> iterator over ChunkPositionAndArea
        running: on_chunk_generated -> position and area separate
        if already in either in_queue or processed, do not add again
        every tick (or every n-th tick?):
            - abort if in_queue is empty
            - convert current_row tiles of in_queue[0] chunk to JSON
            - current_row = (current_row + 1) % 32
            - if current_row == 0 -> move in_queue[0] to back of processed
        once chunk is complete -> remove JSON from global table
    */
}

fn enigo() -> Result<()> {
    let millis = 42;
    let mut enigo = Enigo::new(&Settings::default())?;
    enigo.move_mouse(600, 230, Coordinate::Abs);
    sleep(Duration::from_millis(millis));
    enigo.button(Button::Left, Click);
    sleep(Duration::from_millis(millis));
    enigo.fast_text("hello");
    sleep(Duration::from_millis(millis));
    enigo.button(Button::Left, Press);
    sleep(Duration::from_millis(millis));
    enigo.move_mouse(-30, 0, Coordinate::Rel);
    sleep(Duration::from_millis(millis));
    enigo.key(Key::Delete, Click);
    sleep(Duration::from_millis(millis));
    enigo.button(Button::Left, Release);
    sleep(Duration::from_millis(millis));
    enigo.move_mouse(0, 750, Coordinate::Rel);
    sleep(Duration::from_millis(millis));
    enigo.button(Button::Left, Click);
    sleep(Duration::from_millis(millis));
    enigo.key(Key::UpArrow, Click);
    sleep(Duration::from_millis(millis));

    Ok(())
}

// TODO: put into own file
#[derive(Debug)]
pub enum Source {
    Recipe(String),
    Resource(String),
    Other,
}

fn process_recipes(prototype_stage: &PrototypeStage) {
    let ingredients_and_products = prototype_stage
        .recipes()
        .by_name
        .iter()
        .flat_map(|(_, recipe)| {
            let mut ingredients_and_products =
                recipe.ingredients.keys().cloned().collect::<HashSet<_>>();
            ingredients_and_products.extend(recipe.products.keys().cloned());
            ingredients_and_products
        })
        .collect::<HashSet<_>>();

    let sources = ingredients_and_products
        .iter()
        .map(|name| {
            let mut sources = vec![];
            if let Some(recipes) = prototype_stage.recipes().by_product.get(name) {
                sources.extend(recipes.keys().map(|recipe| Source::Recipe(recipe.clone())));
            }
            if let Some(resource) = prototype_stage.resources().by_name.get(name) {
                sources.push(Source::Resource(resource.name.clone()));
            }
            if sources.is_empty() {
                sources.push(Source::Other);
            }
            (name.clone(), sources)
        })
        .collect::<HashMap<_, _>>();

    println!("{sources:#?}");
}

fn parse_prototype_stage() -> Result<PrototypeStage> {
    let mut archive = ZipArchive::new(File::open("api/json/data-raw-dump-v1.1.101.zip")?)?;
    let mut file = ZipArchive::by_name(&mut archive, "data-raw-dump.json")?;
    let mut data = String::new();

    file.read_to_string(&mut data)?;
    Ok(PrototypeStage::from_str(&data)?)
}

// https://developer.valvesoftware.com/wiki/Source_RCON_Protocol
fn remote_console(runtime_stage: &mut RuntimeStage) -> Result<()> {
    let mut console = RemoteConsole::new("10.243.118.233", 25575, "123")?;
    let setup_response = setup(&mut console)?;
    if !setup_response.is_empty() {
        println!("{setup_response}");
    } else {
        let response = console.send_command(
            "
          Json.to_string(game)
        ",
        )?;
        listen_to_events(&mut console, runtime_stage)?;
        // println!("{response}");
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
                if let Err(e) = &result {
                    println!("{index}.json: {e}");
                    let filename = PathBuf::from(&format!("output/events/{index}.json"));
                    fs::write(filename, event)?;
                    index += 1;
                } else if let Ok(Some(factorio_type)) = result {
                    if let Some(on_event) = factorio_type.as_event() {
                        //if format!("{on_event:?}").starts_with("OnGui") {
                        let filename = PathBuf::from(&format!("output/events/{index}.json"));
                        fs::write(filename, event)?;
                        index += 1;
                        // }
                    }
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
