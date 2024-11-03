#![allow(dead_code)]

use crate::format::Format;
use std::{fs, io};

mod basic_member;
mod concept;
mod custom_property;
mod define;
mod define_value;
mod format;
mod image;
mod pascal_case;
mod property;
mod prototype;
mod type_;

fn deserialize_format() -> io::Result<Format> {
    let json = fs::read_to_string("api/prototype/prototype-api.json")?;
    let format = serde_json::from_str(&json)?;

    Ok(format)
}

pub fn generate_prototype_stage() -> io::Result<()> {
    let format = deserialize_format()?;
    format.generate()?;

    Ok(())
}
