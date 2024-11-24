use crate::lua_defines::parse_lua_defines;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use shared::deserialize_format;
use shared::file_utils::save_file_if_changed;
use std::path::Path;

mod lua_defines;

fn main() -> anyhow::Result<()> {
    let format = deserialize_format(Path::new("../shared/prototype-api.json"))?;
    let lua_defines = parse_lua_defines("defines.lua")?;
    let path = Path::new("src/generated");

    let results = format
        .defines
        .par_iter()
        .map(|define| define.generate(path, &lua_defines))
        .collect::<Vec<_>>();
    results.into_iter().collect::<Result<(), _>>()?;

    let mut content = String::from("pub enum Defines {");
    format.defines.iter().for_each(|define| {
        let rust_name = &define.rust_name();
        content.insert_str(0, &format!("pub mod {};", define.name()));
        content.push_str(&format!("{}({}::{}),", rust_name, define.name(), rust_name));
    });
    let mod_path = &path.join("mod").with_extension("rs");
    save_file_if_changed("defines", mod_path, &format!("{content}}}"))
}
