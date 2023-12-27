use std::{
    env,
    fs::File,
    io::{self, BufReader},
};

use prototype::PrototypeApiFormat;

mod prototype;

fn main() -> io::Result<()> {
    let run_build_script = env::var("RUN_BUILD_SCRIPT")
        .map(|v| v == "1")
        .unwrap_or(true);
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=generated");

    if run_build_script {
        generate_prototype_api()
    } else {
        Ok(())
    }
}

fn generate_prototype_api() -> io::Result<()> {
    let prototype_api_format = read_prototype_api_format("prototype/prototype-api-v1.1.101.json")?;
    prototype_api_format.generate_prototype_api()
}

// https://lua-api.factorio.com/1.1.101/index-prototype.html
fn read_prototype_api_format(json_path: &str) -> io::Result<PrototypeApiFormat> {
    let file = File::open(json_path)?;
    let reader = BufReader::new(file);
    let prototype_api_format = serde_json::from_reader(reader)?;

    Ok(prototype_api_format)
}
