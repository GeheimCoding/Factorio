#![allow(unused)]
#![deny(clippy::unwrap_used)]

use api::{load_data_dump, FactorioType};
use remote_console::RemoteConsole;
use std::fs;

fn main() -> anyhow::Result<()> {
    // remote_console()?;

    let data = load_data_dump()?;
    println!("{:#?}", data);

    Ok(())
}

// https://developer.valvesoftware.com/wiki/Source_RCON_Protocol
fn remote_console() -> anyhow::Result<()> {
    let mut console = RemoteConsole::new("10.243.166.195", 25575, "123")?;
    let mut setup = fs::read_to_string("setup/setup.lua")?;
    let response = console.send_command(&setup)?;
    if !response.is_empty() {
        println!("{response}");
    } else {
        todo!()
    }

    Ok(())
}
