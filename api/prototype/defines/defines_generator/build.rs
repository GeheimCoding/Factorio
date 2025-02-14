use crate::lua_defines::parse_lua_defines;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use shared::deserialize_format;
use shared::file_utils::save_file_if_changed;
use std::path::Path;

mod lua_defines;

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=../../../generated/src/defines");

    let format = deserialize_format(Path::new("../../prototype-api.json"))?;
    let lua_defines = parse_lua_defines("defines.lua")?;
    let path = Path::new("../../../generated/src/defines");

    let results = format
        .defines
        .par_iter()
        .map(|define| define.generate(path, &lua_defines))
        .collect::<Vec<_>>();
    results.into_iter().collect::<Result<(), _>>()?;

    let mut content = String::from("pub enum Defines {");
    format.defines.iter().for_each(|define| {
        let rust_name = &define.rust_name();
        let define_name = define.name();
        content.insert_str(
            0,
            &format!("pub mod {define_name};pub use {define_name}::{rust_name};"),
        );
        content.push_str(&format!("{rust_name}({rust_name}),"));
    });
    content.insert_str(0, "#![allow(dead_code)]");
    let mod_path = &path.join("mod").with_extension("rs");
    save_file_if_changed("defines", mod_path, &format!("{content}}}"))
}
