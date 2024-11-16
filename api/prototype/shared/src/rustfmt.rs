use std::io;
use std::path::Path;
use std::process::{Child, Command};

pub fn rustfmt(path: &Path) -> io::Result<Child> {
    Command::new("rustfmt").arg(path).spawn()
}
