use crate::prototype_format::PrototypeFormat;
use crate::runtime_format::RuntimeFormat;
use std::fs;
use std::path::Path;

pub mod attribute;
pub mod basic_member;
pub mod class;
pub mod context;
pub mod custom_properties;
pub mod define;
pub mod define_value;
pub mod derive;
pub mod event;
pub mod event_raised;
pub mod file_utils;
pub mod image;
pub mod lua_value;
pub mod method;
pub mod method_format;
pub mod operator;
pub mod parameter;
pub mod parameter_group;
pub mod property;
pub mod prototype;
pub mod prototype_concept;
pub mod prototype_format;
pub mod runtime_concept;
pub mod runtime_format;
pub mod transformation;
pub mod type_;
pub mod variadic_parameter;

pub fn deserialize_format(path: &Path) -> anyhow::Result<PrototypeFormat> {
    let json = fs::read_to_string(path)?;
    let format = serde_json::from_str(&json)?;

    Ok(format)
}

pub fn deserialize_runtime_format(path: &Path) -> anyhow::Result<RuntimeFormat> {
    let json = fs::read_to_string(path)?;
    let format = serde_json::from_str(&json)?;

    Ok(format)
}
