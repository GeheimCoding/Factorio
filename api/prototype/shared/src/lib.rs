use crate::format::Format;
use std::fs;
use std::path::Path;

pub mod basic_member;
pub mod concept;
pub mod custom_properties;
pub mod define;
pub mod define_value;
pub mod derive;
pub mod file_utils;
pub mod format;
pub mod image;
pub mod lua_value;
pub mod property;
pub mod prototype;
pub mod transformation;
pub mod type_;

pub fn deserialize_format(path: &Path) -> anyhow::Result<Format> {
    let json = fs::read_to_string(path)?;
    let format = serde_json::from_str(&json)?;

    Ok(format)
}
