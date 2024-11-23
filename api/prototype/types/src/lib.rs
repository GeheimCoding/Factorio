use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use shared::deserialize_format;
use shared::file_utils::save_file_if_changed;
use std::path::Path;

pub mod generated;

pub fn build_types() -> anyhow::Result<()> {
    let format = deserialize_format(Path::new("api/prototype/shared/prototype-api.json"))?;
    let path = Path::new("api/prototype/types/src/generated");

    let mut content = String::from("pub enum Types {");
    format
        .types
        .iter()
        .filter(|concept| !concept.is_builtin())
        .for_each(|concept| {
            let rust_name = &concept.rust_name();
            content.insert_str(0, &format!("pub mod {};", concept.name()));
            content.push_str(&format!(
                "{}({}::{}),",
                rust_name,
                concept.name(),
                rust_name
            ));
        });
    let mod_path = &path.join("mod").with_extension("rs");
    save_file_if_changed("types", mod_path, &format!("{content}}}"))?;

    let results = format
        .types
        .par_iter()
        .filter(|concept| !concept.is_builtin())
        .map(|concept| concept.generate(path))
        .collect::<Vec<_>>();
    results.into_iter().collect::<Result<_, _>>()
}
