#![allow(unused)]
#![deny(clippy::unwrap_used)]

//mod generated;
mod remote_console;

use std::{collections::HashMap, fs, io};

use remote_console::RemoteConsole;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct ItemStackDefinition {
    pub ammo: Option<f64>,
    pub count: Option<u32>,
    pub durability: Option<f64>,
    pub health: Option<f32>,
    pub name: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum SimpleItemStack {
    String(String),
    ItemStackDefinition(ItemStackDefinition),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "serde_tag")]
enum EventUnion {
    OnPlayerMinedItem(OnPlayerMinedItem),
    OnPlayerMinedTile(OnPlayerMinedTile),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Events {
    OnPlayerMinedItem,
    OnPlayerMinedTile,
}

#[derive(Debug, Deserialize)]
struct OnPlayerMinedItem {
    pub item_stack: SimpleItemStack,
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

#[derive(Debug, Deserialize)]
struct OnPlayerMinedTile {
    pub name: Events,
    pub player_index: u32,
    pub tick: u32,
}

#[derive(Debug, Deserialize)]
enum ClassUnion {}

#[derive(Debug, Deserialize)]
enum ConceptUnion {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "serde_type")]
enum FactorioType {
    Class(ClassUnion),
    Concept(ConceptUnion),
    Event(EventUnion),
}

fn main() -> io::Result<()> {
    remote_console()?;
    Ok(())
}

fn main_old() -> io::Result<()> {
    let mut table = HashMap::new();
    table.insert(8, "on_player_mined_item");
    table.insert(48, "on_player_mined_tile");
    let json = r#"{
        "item_stack": {
            "count": 1,
            "name": "stone"
        },
        "name": 8,
        "player_index": 1,
        "tick": 773
    }"#;
    let mut json_value: Value = serde_json::from_str(json).unwrap();
    if let Value::Object(obj) = &mut json_value {
        let name = obj.get("name").unwrap();
        let name = (*table.get(&name.as_u64().unwrap()).unwrap()).to_owned();
        obj.insert("name".to_owned(), Value::String(name.clone()));
        obj.insert("serde_tag".to_owned(), Value::String(name));
        obj.insert("serde_type".to_owned(), Value::String("event".to_owned()));
    }

    let typ: Result<FactorioType, _> = serde_json::from_value(json_value);
    println!("{typ:?}");

    Ok(())
}

// https://developer.valvesoftware.com/wiki/Source_RCON_Protocol
fn remote_console() -> io::Result<()> {
    let mut console = RemoteConsole::new("10.243.166.195", 25575, "123")?;
    let to_json = fs::read_to_string("setup.lua")?;
    let response = console.send_command(&to_json)?;
    if !response.is_empty() {
        println!("{response}");
    } else {
        let response = console.send_command(
            "
            local map = {}
            --rcon.print(to_json(game.item_prototypes['wood'].group.subgroups, 1, map))
            --rcon.print(to_json(game, 1, map))
            rcon.print(to_json(game.item_prototypes['wood'].group, 1, map))
            --to_json(game, 1, map)
            rcon.print(serpent.block(map))
            print('done')
        ",
        )?;
        println!("{response}");
    }

    Ok(())
}

// https://wiki.factorio.com/Materials_and_recipes
// TODO: LuaGroup type item-group = root (= section in crafting window), subgroup is one line in crafting window

// TODO: improve performance of lookup table with grouping (e.g. by object_name)?
//      -> currently around 20 seconds without LuaGroup and cycles for game
// TODO: split map_settings and other tables in separate files
// TODO: check for more "cycles"
// TODO: use global lookup table for objects and unique_ids
// TODO: make subclass specific attributes optional?
// TODO: fix/confirm subclasses type casing
// TODO: convert array to json array
// TODO: fix trailing commmas
// TODO: add serde tags
// TODO: implement TODOs
// TODO: improve performance
