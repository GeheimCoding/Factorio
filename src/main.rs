#![allow(unused)]
#![deny(clippy::unwrap_used)]

mod remote_console;
mod runtime_api_format;

use std::{
    fs::{self, File},
    io::{self, BufReader},
};

use remote_console::RemoteConsole;
use runtime_api_format::RuntimeApiFormat;

fn main() -> io::Result<()> {
    // TODO: move to build.rs
    generate_runtime_api()
}

fn generate_runtime_api() -> io::Result<()> {
    let runtime_api_format = read_runtime_api_format("runtime-api.json")?;
    runtime_api_format.generate_runtime_api()
}

// https://lua-api.factorio.com/latest/json-docs.html
fn read_runtime_api_format(json_path: &str) -> io::Result<RuntimeApiFormat> {
    let file = File::open(json_path)?;
    let reader = BufReader::new(file);
    let runtime_api_format = serde_json::from_reader(reader)?;

    Ok(runtime_api_format)
}

// https://developer.valvesoftware.com/wiki/Source_RCON_Protocol
fn remote_console() -> io::Result<()> {
    let mut console = RemoteConsole::new("10.243.166.195", 25575, "123")?;
    let response = console.send_command("rcon.print('Hello Tomo from Factorio!')")?;
    println!("{response}");

    Ok(())
}
