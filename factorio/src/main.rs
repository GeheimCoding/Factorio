#![allow(unused)]
#![deny(clippy::unwrap_used)]

use std::any::{Any, TypeId};
use std::fmt::Debug;
use std::{collections::HashMap, fs, io, path::PathBuf, thread::sleep, time::Duration};

use anyhow::{Context, Result};
use struct_iterable::Iterable;

use api::{
    add_to_lua_objects, parse_factorio_type, FactorioType, LuaAISettings, LuaRecipe,
    LuaRecipePrototype, MaybeCycle,
};
use remote_console::RemoteConsole;

fn main() -> Result<()> {
    //remote_console()?;
    // parse_recipes().context("parse_recipes")?;
    let content = fs::read_to_string("output/game.json")?;
    let game = parse_factorio_type(&content)?;
    let mut lua_objects = HashMap::new();

    add_to_lua_objects(
        game.as_class().unwrap().as_lua_game_script().unwrap(),
        &mut lua_objects,
    );
    println!("{:?}", lua_objects.keys());

    Ok(())
}

fn parse_recipes() -> Option<()> {
    let content = fs::read_to_string("output/game.json").ok()?;
    let game = parse_factorio_type(&content).ok()?;

    let recipes = get_recipes(&game)?;
    let mut recipes = recipes.iter().map(|(k, v)| k).cloned().collect::<Vec<_>>();
    recipes.sort();

    println!("{}", recipes.len());
    println!("{}", get_recipe_prototypes(&game)?.len());
    println!("{}", recipes.join("\n"));

    Some(())
}

// https://wiki.factorio.com/Materials_and_recipes
fn get_recipes(factorio_type: &FactorioType) -> Option<&HashMap<String, MaybeCycle<LuaRecipe>>> {
    Some(
        &factorio_type
            .as_class()?
            .as_lua_game_script()?
            .forces
            .get(&api::LuaGameScriptForces::String("player".to_owned()))?
            .as_value()?
            .recipes,
    )
}

fn get_recipe_prototypes(
    factorio_type: &FactorioType,
) -> Option<&HashMap<String, MaybeCycle<LuaRecipePrototype>>> {
    Some(
        &factorio_type
            .as_class()?
            .as_lua_game_script()?
            .recipe_prototypes,
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

fn listen_to_events(console: &mut RemoteConsole) -> Result<()> {
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
                let factorio_type = parse_factorio_type(event);
                if let Err(e) = factorio_type {
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
