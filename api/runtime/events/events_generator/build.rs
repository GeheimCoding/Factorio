use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use shared::deserialize_runtime_format;
use shared::file_utils::save_file_if_changed;
use std::path::Path;

pub fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=../../../generated/src/events");

    let format = deserialize_runtime_format(Path::new("../../runtime-api.json"))?;
    let path = Path::new("../../../generated/src/events");
    let context = format.create_context();

    let results = format
        .events
        .par_iter()
        .map(|event| event.generate(path, &context))
        .collect::<Vec<_>>();
    results.into_iter().collect::<Result<(), _>>()?;

    let mut content = String::from(
        "#[derive(Debug, serde::Deserialize)]#[serde(tag = \"serde_tag\")]pub enum Events {",
    );
    format.events.iter().for_each(|event| {
        let rust_name = &event.rust_name();
        let event_name = event.name();
        content.insert_str(
            0,
            &format!("pub mod {event_name};pub use {event_name}::{rust_name};"),
        );
        content.push_str(&format!("{rust_name}(Box<{rust_name}>),",));
    });
    content.insert_str(0, "#![allow(dead_code)]");
    let mod_path = &path.join("mod").with_extension("rs");
    save_file_if_changed("events", mod_path, &format!("{content}}}"))
}
