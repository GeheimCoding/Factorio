use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use shared::file_utils::save_file;
use shared::format::Format;
use std::path::Path;
use std::{fs, io};

pub mod generated;

// TODO: pass defines from game to set the correct value per variant -> print(serpent.block(defines))
pub fn generate(format: &Format) -> io::Result<()> {
    let path = Path::new("api/prototype/defines/src/generated");
    fs::create_dir_all(path)?;

    let mut content = String::from("pub enum Defines {");
    format.defines.iter().for_each(|define| {
        let rust_name = &define.rust_name();
        content.insert_str(0, &format!("pub mod {};", define.name()));
        content.push_str(&format!("{}({}::{}),", rust_name, define.name(), rust_name));
    });
    let mod_path = &path.join("mod").with_extension("rs");
    save_file(mod_path, &format!("{content}}}"))?;

    let results = format
        .defines
        .par_iter()
        .map(|define| define.generate(path))
        .collect::<Vec<_>>();
    results.into_iter().collect::<Result<_, _>>()
}
