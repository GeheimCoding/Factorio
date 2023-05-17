#![deny(clippy::unwrap_used)]

use std::{
    fs::File,
    io::{self, BufReader},
};

mod runtime_api_format;
use runtime_api_format::RuntimeApiFormat;

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=generated");
    generate_runtime_api()
}

fn generate_runtime_api() -> io::Result<()> {
    let runtime_api_format = read_runtime_api_format("runtime_api_format/runtime-api.json")?;
    runtime_api_format.generate_runtime_api()
}

// https://lua-api.factorio.com/latest/json-docs.html
fn read_runtime_api_format(json_path: &str) -> io::Result<RuntimeApiFormat> {
    let file = File::open(json_path)?;
    let reader = BufReader::new(file);
    let runtime_api_format = serde_json::from_reader(reader)?;

    Ok(runtime_api_format)
}
