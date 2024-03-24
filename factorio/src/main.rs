#![allow(unused)]
#![deny(clippy::unwrap_used)]

use std::any::{Any, TypeId};
use std::collections::HashSet;
use std::fmt::Debug;
use std::rc::Rc;
use std::{collections::HashMap, fs, io, path::PathBuf, thread::sleep, time::Duration};

use anyhow::{Context, Result};

use api::{
    parse_factorio_type, FactorioType, LuaAISettings, LuaAchievementPrototype, LuaGameScript,
    LuaGroup, LuaObjects, LuaRecipe, LuaRecipePrototype, MaybeCycle, Runtime,
};
use extensions::{LuaObject, Traversable};
use remote_console::RemoteConsole;

fn main() -> Result<()> {
    let mut runtime = Runtime::new();
    // remote_console()?;
    // parse_recipes(&mut lua_objects).context("parse_recipes")?;

    let content = fs::read_to_string("output/game.json")?;
    let game = parse_factorio_type(&content, &mut runtime)?;

    use_lua_object("65", runtime.lua_objects()).context("use_lua_object")?;
    println!("{}", runtime.factorio_types().len());

    Ok(())
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
