use crate::format::Format;
use std::path::Path;
use std::{fs, io};

pub mod basic_member;
pub mod concept;
pub mod custom_property;
pub mod define;
pub mod define_value;
pub mod file_utils;
pub mod format;
pub mod image;
pub mod lua_value;
pub mod pascal_case;
pub mod property;
pub mod prototype;
pub mod type_;

pub fn deserialize_format(path: &Path) -> io::Result<Format> {
    let json = fs::read_to_string(path)?;
    let format = serde_json::from_str(&json)?;

    Ok(format)
}
