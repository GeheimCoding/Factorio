#![allow(unused)]
#![deny(clippy::unwrap_used)]

mod remote_console;
mod runtime_api;

use std::{
    fs::{self, File},
    io::{self, BufReader},
};

use remote_console::RemoteConsole;
use runtime_api::{Class, RuntimeApi};

fn main() -> io::Result<()> {
    let runtime_api = read_runtime_api("runtime-api.json")?;

    // for class in runtime_api.classes {
    //     //if class.name == "LuaAccumulatorControlBehavior" {
    //     println!("{class}");
    //     //}
    // }
    for define in runtime_api.defines {
        print!("{}", define.get_definitions(""));
    }

    // TODO: resolve concepts and defines
    // for concept in runtime_api.concepts {
    //     println!("{}", concept.typ);
    // }

    Ok(())
}

// https://lua-api.factorio.com/latest/json-docs.html
fn read_runtime_api(json_path: &str) -> io::Result<RuntimeApi> {
    let file = File::open(json_path)?;
    let reader = BufReader::new(file);
    let runtime_api = serde_json::from_reader(reader)?;

    Ok(runtime_api)
}

// https://developer.valvesoftware.com/wiki/Source_RCON_Protocol
fn remote_console() -> io::Result<()> {
    let mut console = RemoteConsole::new("192.168.1.108", 25575, "123")?;
    let response = console.send_command("rcon.print('Hello Tomo from Factorio!')")?;
    println!("{response}");

    Ok(())
}
