use anyhow::Context;
use std::env::temp_dir;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Child, Command};

pub fn save_file_if_changed(parent: &str, path: &Path, content: &str) -> anyhow::Result<()> {
    create_all_directories(path)?;
    if has_content_changed(parent, path, content)? {
        fs::write(path, content)?;
        if path.extension() == Some(OsStr::new("rs")) {
            rustfmt(path)?;
        }
    }
    Ok(())
}

fn create_all_directories(path: &Path) -> anyhow::Result<()> {
    let dir_path = if path.is_dir() {
        path
    } else {
        path.parent().context("parent")?
    };
    fs::create_dir_all(dir_path).context(format!("failed to create directory at {:?}", path))
}

fn has_content_changed(parent: &str, path: &Path, content: &str) -> anyhow::Result<bool> {
    let temp_dir = &temp_dir().join(parent);
    fs::create_dir_all(temp_dir)?;

    let temp_file_path = &temp_dir.join(path.file_name().context("file_name")?);
    let temp_file = fs::File::create(temp_file_path)?;
    write!(&temp_file, "{content}")?;
    if temp_file_path.extension() == Some(OsStr::new("rs")) {
        rustfmt(temp_file_path)?.wait()?;
    }
    let content = fs::read_to_string(temp_file_path)?;
    let existing = fs::read_to_string(path).unwrap_or_default();
    Ok(content != existing)
}

fn rustfmt(path: &Path) -> anyhow::Result<Child> {
    Command::new("rustfmt")
        .arg(path)
        .spawn()
        .context("failed to run rustfmt")
}
