use crate::concept::Concept;
use crate::define::Define;
use crate::prototype::Prototype;
use serde::Deserialize;
use std::path::Path;
use std::process::{Child, Command};
use std::{fs, io};

// https://lua-api.factorio.com/stable/auxiliary/json-docs-prototype.html
#[derive(Debug, Deserialize)]
pub struct Format {
    application: String,
    application_version: String,
    api_version: u8,
    stage: String,
    prototypes: Vec<Prototype>,
    types: Vec<Concept>,
    defines: Vec<Define>,
}

impl Format {
    pub fn generate(&self) -> io::Result<()> {
        let defines = self.generate_defines();
        let path = Path::new("api/prototype/generated/defines.rs");
        fs::create_dir_all(path.parent().expect("parent directory"))?;
        fs::write("api/prototype/generated/defines.rs", defines)?;
        Self::rustfmt(path)?;

        Ok(())
    }

    fn generate_defines(&self) -> String {
        let mut defines = String::new();
        for define in &self.defines {
            defines.push_str(&define.generate())
        }
        defines
    }

    fn rustfmt(path: &Path) -> io::Result<Child> {
        Command::new("rustfmt").arg(path).spawn()
    }
}
