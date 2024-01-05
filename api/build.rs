use std::{
    env,
    fs::{self, File},
    io::{self, BufReader},
    process::{Child, Command},
};

use generator::{prototype::api_format::PrototypeApiFormat, runtime::api_format::RuntimeApiFormat};

mod generator;

fn main() -> io::Result<()> {
    let run_build_script = env::var("RUN_BUILD_SCRIPT")
        .map(|v| v == "1")
        .unwrap_or(true);
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=generated");

    if run_build_script {
        fs::create_dir_all("src/generated")?;
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

fn generate_runtime_api() -> io::Result<Child> {
    let classes_path = "src/generated/classes.rs";
    let events_path = "src/generated/events.rs";
    let concepts_path = "src/generated/concepts.rs";
    let defines_path = "src/generated/defines.rs";

    read_runtime_api_format("json/runtime-api-v1.1.101.json")?.generate_runtime_api(
        classes_path,
        events_path,
        concepts_path,
        defines_path,
    )?;
    rustfmt(defines_path)
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
