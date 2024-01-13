#![allow(unused)]
#![deny(clippy::unwrap_used)]

use api;
use remote_console::RemoteConsole;
use std::{fs, io};

fn main() -> io::Result<()> {
    remote_console()?;
    Ok(())
}

// https://developer.valvesoftware.com/wiki/Source_RCON_Protocol
fn remote_console() -> io::Result<()> {
    let mut console = RemoteConsole::new("10.243.118.233", 25575, "123")?;
    let response = console.send_command(&fs::read_to_string("factorio/src/setup.lua")?)?;
    if !response.is_empty() {
        println!("{response}");
    } else {
        let response = console.send_command(
            "
            rcon.print(Json.parse(game.forces))
        ",
        )?;
        println!("{response}");
    }
    Ok(())
}
