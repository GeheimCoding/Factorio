use std::env::temp_dir;
use std::ffi::OsStr;
use std::io::Write;
use std::path::Path;
use std::process::{Child, Command};
use std::{fs, io};

pub fn save_file_if_changed(parent: &str, path: &Path, content: &str) -> io::Result<()> {
    create_all_directories(path)?;
    if has_content_changed(parent, &path, content)? {
        fs::write(path, content)?;
        if path.extension() == Some(OsStr::new("rs")) {
            rustfmt(path)?;
        }
    }
    Ok(())
}

fn create_all_directories(path: &Path) -> io::Result<()> {
    if path.is_dir() {
        fs::create_dir_all(path)
    } else {
        fs::create_dir_all(path.parent().expect("parent"))
    }
}

fn has_content_changed(parent: &str, path: &Path, content: &str) -> io::Result<bool> {
    let temp_dir = &temp_dir().join(parent);
    fs::create_dir_all(temp_dir)?;

    let temp_file_path = &temp_dir.join(path.file_name().expect("file_name"));
    let temp_file = fs::File::create(temp_file_path)?;
    write!(&temp_file, "{content}")?;
    if temp_file_path.extension() == Some(OsStr::new("rs")) {
        rustfmt(&temp_file_path)?.wait()?;
    }
    let content = fs::read_to_string(temp_file_path)?;
    let existing = fs::read_to_string(path).unwrap_or(String::new());
    Ok(content != existing)
}

fn rustfmt(path: &Path) -> io::Result<Child> {
    Command::new("rustfmt").arg(path).spawn()
}
