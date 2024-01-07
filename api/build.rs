use std::{
    env,
    fs::{self, File},
    io::{self, BufReader},
    process::{Child, Command},
};

use generator::{
    generate_factorio_types, generate_mod, prototype::api_format::PrototypeApiFormat,
    runtime::api_format::RuntimeApiFormat,
};

mod generator;

fn main() -> io::Result<()> {
    let run_build_script = env::var("RUN_BUILD_SCRIPT")
        .map(|v| v == "1")
        .unwrap_or(true);
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=generated");

    if run_build_script {
        let base_path = "src/generated";
        fs::create_dir_all(base_path)?;
        let prototype_api = generate_prototype_api(base_path)?;
        let runtime_api = generate_runtime_api(base_path)?;
        generate_lib(base_path, &prototype_api, &runtime_api)?;
    }
    Ok(())
}

fn generate_prototype_api(base_path: &str) -> io::Result<PrototypeApiFormat> {
    let prototypes_path = &format!("{base_path}/prototypes.rs");
    let types_path = &format!("{base_path}/types.rs");

    let prototype_api = read_prototype_api_format("json/prototype-api-v1.1.101.json")?;
    prototype_api.generate_prototype_api(prototypes_path, types_path)?;

    rustfmt(prototypes_path)?;
    rustfmt(types_path)?;
    Ok(prototype_api)
}

fn generate_runtime_api(base_path: &str) -> io::Result<RuntimeApiFormat> {
    let classes_path = &format!("{base_path}/classes.rs");
    let events_path = &format!("{base_path}/events.rs");
    let concepts_path = &format!("{base_path}/concepts.rs");
    let defines_path = &format!("{base_path}/defines.rs");

    let runtime_api = read_runtime_api_format("json/runtime-api-v1.1.101.json")?;
    runtime_api.generate_runtime_api(classes_path, events_path, concepts_path, defines_path)?;

    rustfmt(classes_path)?;
    rustfmt(events_path)?;
    rustfmt(concepts_path)?;
    rustfmt(defines_path)?;
    Ok(runtime_api)
}

fn generate_lib(
    base_path: &str,
    prototype_api: &PrototypeApiFormat,
    runtime_api: &RuntimeApiFormat,
) -> io::Result<Child> {
    let factorio_types_path = &format!("{base_path}/factorio_types.rs");
    let mod_path = &format!("{base_path}/mod.rs");

    generate_factorio_types(factorio_types_path, prototype_api, runtime_api)?;
    generate_mod(mod_path)?;
    rustfmt(mod_path)
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
