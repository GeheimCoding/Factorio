#![deny(clippy::unwrap_used)]

use std::{
    env,
    fs::File,
    io::{self, BufReader},
};

mod runtime_api_format;
use runtime_api_format::RuntimeApiFormat;

fn main() -> io::Result<()> {
    let run_build_script = env::var("RUN_BUILD_SCRIPT")
        .map(|v| v == "1")
        .unwrap_or(true);
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=generated");

    if run_build_script {
        generate_runtime_api()
    } else {
        Ok(())
    }
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
