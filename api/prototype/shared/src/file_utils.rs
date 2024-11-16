use std::path::Path;
use std::process::{Child, Command};
use std::{fs, io};

pub fn save_file(path: &Path, content: &str) -> io::Result<()> {
    fs::write(path, content)?;
    rustfmt(path)?;
    Ok(())
}

fn rustfmt(path: &Path) -> io::Result<Child> {
    Command::new("rustfmt").arg(path).spawn()
}
