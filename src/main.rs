use std::io;

use crate::remote_console::RemoteConsole;
mod remote_console;

// https://developer.valvesoftware.com/wiki/Source_RCON_Protocol
fn main() -> io::Result<()> {
    let mut console = RemoteConsole::new("192.168.1.108", 25575, "123")?;
    let response = console.send_command("rcon.print('Hello Tomo from Factorio!')")?;
    println!("{response}");

    Ok(())
}
