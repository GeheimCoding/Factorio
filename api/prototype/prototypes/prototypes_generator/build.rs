use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use shared::deserialize_format;
use shared::file_utils::save_file_if_changed;
use std::path::Path;

pub fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=../../generated/src/prototypes");

    let format = deserialize_format(Path::new("../../shared/prototype-api.json"))?;
    let path = Path::new("../../generated/src/prototypes");
    let context = format.create_context();

    let results = format
        .prototypes
        .par_iter()
        .map(|prototype| prototype.generate(path, &context))
        .collect::<Vec<_>>();
    results.into_iter().collect::<Result<(), _>>()?;

    let mut content = String::from("pub enum Prototypes {");
    format.prototypes.iter().for_each(|prototype| {
        let rust_name = &prototype.rust_name();
        let prototype_name = prototype.name();
        content.insert_str(
            0,
            &format!("pub mod {prototype_name};pub use {prototype_name}::{rust_name};"),
        );
        content.push_str(&format!("{rust_name}(Box<{rust_name}>),",));
    });
    let mod_path = &path.join("mod").with_extension("rs");
    save_file_if_changed("prototypes", mod_path, &format!("{content}}}"))
}
