use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use shared::concept::{Concept, Kind};
use shared::deserialize_format;
use shared::file_utils::save_file_if_changed;
use std::collections::HashMap;
use std::path::Path;

//pub mod generated;

pub fn build_types() -> anyhow::Result<()> {
    let format = deserialize_format(Path::new("api/prototype/shared/prototype-api.json"))?;
    let path = Path::new("api/prototype/types/src/generated");
    let kinds = create_kinds_lookup(&format.types);

    let results = format
        .types
        .par_iter()
        .filter(|concept| concept.should_be_generated())
        .map(|concept| concept.generate(path, &kinds))
        .collect::<Vec<_>>();
    results.into_iter().collect::<Result<(), _>>()?;

    let mut content = String::from("pub enum Types {");
    format
        .types
        .iter()
        .filter(|concept| concept.should_be_generated())
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
    save_file_if_changed("types", mod_path, &format!("{content}}}"))
}

fn create_kinds_lookup(concepts: &Vec<Concept>) -> HashMap<String, Kind> {
    let mut kinds = HashMap::new();
    concepts.iter().for_each(|concept| {
        let name = concept.rust_name();
        let kind = if concept.type_.is_struct() {
            Kind::Struct
        } else if concept.type_.is_union() {
            Kind::Union
        } else {
            Kind::NewType
        };
        kinds.insert(String::from(name), kind);
    });
    kinds
}
