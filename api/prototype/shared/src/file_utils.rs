use std::ffi::OsStr;
use std::path::Path;
use std::process::{Child, Command};
use std::{fs, io};

pub fn save_file_if_changed(path: &Path, content: &str) -> io::Result<()> {
    create_all_directories(path)?;
    let existing = fs::read_to_string(path).unwrap_or(String::new());
    if has_content_changed(content, &existing) {
        fs::write(path, content)?;
        if path.extension() == Some(OsStr::new("rs")) {
            rustfmt(path)?;
        }
    }
    Ok(())
}

pub fn create_rustfmt_config(path: &Path, options: &[&str]) -> io::Result<()> {
    save_file_if_changed(
        &path.join("rustfmt").with_extension("toml"),
        &options.join("\n"),
    )
}

pub fn create_rustfmt_config_without_reordering(path: &Path) -> io::Result<()> {
    create_rustfmt_config(
        path,
        &["reorder_imports = false", "reorder_modules = false"],
    )
}

fn create_all_directories(path: &Path) -> io::Result<()> {
    if path.is_dir() {
        fs::create_dir_all(path)
    } else {
        fs::create_dir_all(path.parent().expect("parent"))
    }
}

fn has_content_changed(c1: &str, c2: &str) -> bool {
    c1.split_whitespace().collect::<String>() != c2.split_whitespace().collect::<String>()
}

fn rustfmt(path: &Path) -> io::Result<Child> {
    Command::new("rustfmt").arg(path).spawn()
}
