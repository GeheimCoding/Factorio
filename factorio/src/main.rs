#![allow(unused)]
#![deny(clippy::unwrap_used)]

use api::{
    parse_factorio_type, LuaEntityPrototype, LuaGameScript, LuaItemPrototype, LuaParticlePrototype,
    LuaTilePrototype, MaybeCycle,
};
use remote_console::RemoteConsole;
use std::{collections::HashMap, fs, io};

fn main() -> io::Result<()> {
    remote_console()?;
    Ok(())
}

// https://developer.valvesoftware.com/wiki/Source_RCON_Protocol
fn remote_console() -> io::Result<()> {
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
        println!("{response}");
        let game = parse_factorio_type(&response)?;
        println!("{game:#?}");
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
