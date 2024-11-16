use shared::format::Format;
use std::{fs, io};

pub fn generate_prototype_stage() -> io::Result<()> {
    let format = deserialize_format()?;

    defines::generate(&format)
}

fn deserialize_format() -> io::Result<Format> {
    let json = fs::read_to_string("api/prototype/prototype-api.json")?;
    let format = serde_json::from_str(&json)?;

    Ok(format)
}
