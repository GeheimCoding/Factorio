use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use shared::deserialize_format;
use shared::file_utils::save_file_if_changed;
use std::path::Path;

pub fn build_types() -> anyhow::Result<()> {
    let format = deserialize_format(Path::new("api/prototype/shared/prototype-api.json"))?;
    let path = Path::new("api/prototype/generated/src/types");
    let context = format.create_context();

    let results = format
        .types
        .par_iter()
        .filter(|concept| concept.should_be_generated())
        .map(|concept| concept.generate(path, &context))
        .collect::<Vec<_>>();
    results.into_iter().collect::<Result<(), _>>()?;

    let mut content = String::from("pub enum Types {");
    format
        .types
        .iter()
        .filter(|concept| concept.should_be_generated())
        .for_each(|concept| {
            let rust_name = &concept.rust_name();
            let concept_name = concept.name();
            content.insert_str(
                0,
                &format!("pub mod {concept_name};pub use {concept_name}::{rust_name};"),
            );
            content.push_str(&format!("{rust_name}(Box<{rust_name}>),",));
        });
    let mod_path = &path.join("mod").with_extension("rs");
    save_file_if_changed("types", mod_path, &format!("{content}}}"))
}
