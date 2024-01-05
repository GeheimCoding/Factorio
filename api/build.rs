use std::{
    env,
    fs::File,
    io::{self, BufReader},
    process::{Child, Command},
};

use generator::{prototype::api_format::PrototypeApiFormat, runtime::RuntimeApiFormat};

mod generator;

fn main() -> io::Result<()> {
    let run_build_script = env::var("RUN_BUILD_SCRIPT")
        .map(|v| v == "1")
        .unwrap_or(true);
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=generated");

    if run_build_script {
        generate_prototype_api()?;
        generate_runtime_api()?;
    }
    Ok(())
}

fn generate_prototype_api() -> io::Result<Child> {
    let prototypes_path = "src/generated/prototypes.rs";
    let types_path = "src/generated/types.rs";

    read_prototype_api_format("json/prototype-api-v1.1.101.json")?
        .generate_prototype_api(prototypes_path, types_path)?;
    rustfmt(prototypes_path)?;
    rustfmt(types_path)
}

fn generate_runtime_api() -> io::Result<()> {
    read_runtime_api_format("json/runtime-api-v1.1.101.json")?.generate_runtime_api()
}

// https://lua-api.factorio.com/1.1.101/index-prototype.html
fn read_prototype_api_format(json_path: &str) -> io::Result<PrototypeApiFormat> {
    let file = File::open(json_path)?;
    let reader = BufReader::new(file);
    let prototype_api_format = serde_json::from_reader(reader)?;

    Ok(prototype_api_format)
}

// https://lua-api.factorio.com/1.1.101/index-runtime.html
fn read_runtime_api_format(json_path: &str) -> io::Result<RuntimeApiFormat> {
    let file = File::open(json_path)?;
    let reader = BufReader::new(file);
    let runtime_api_format = serde_json::from_reader(reader)?;

    Ok(runtime_api_format)
}

fn rustfmt(path: &str) -> io::Result<Child> {
    Command::new("rustfmt").arg(path).spawn()
}
