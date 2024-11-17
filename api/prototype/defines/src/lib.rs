use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use shared::file_utils::{create_rustfmt_config_without_reordering, save_file_if_changed};
use shared::format::Format;
use std::io;
use std::path::Path;

//pub mod generated;

// TODO: pass defines from game to set the correct value per variant -> print(serpent.block(defines))
pub fn generate(format: &Format) -> io::Result<()> {
    let path = Path::new("api/prototype/defines/src/generated");
    create_rustfmt_config_without_reordering(path)?;

    let mut content = String::from("pub enum Defines {");
    format.defines.iter().for_each(|define| {
        let rust_name = &define.rust_name();
        content.insert_str(0, &format!("pub mod {};", define.name()));
        content.push_str(&format!("{}({}::{}),", rust_name, define.name(), rust_name));
    });
    let mod_path = &path.join("mod").with_extension("rs");
    save_file_if_changed(mod_path, &format!("{content}}}"))?;

    let results = format
        .defines
        .par_iter()
        .map(|define| define.generate(path))
        .collect::<Vec<_>>();
    results.into_iter().collect::<Result<_, _>>()
}
