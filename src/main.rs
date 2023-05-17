#![allow(unused)]
#![deny(clippy::unwrap_used)]

//mod generated;
mod remote_console;

use std::io;

use remote_console::RemoteConsole;

fn main() -> io::Result<()> {
    Ok(())
}

// https://developer.valvesoftware.com/wiki/Source_RCON_Protocol
fn remote_console() -> io::Result<()> {
    let mut console = RemoteConsole::new("10.243.166.195", 25575, "123")?;
    let response = console.send_command("rcon.print('Hello Tomo from Factorio!')")?;
    println!("{response}");

    Ok(())
}
