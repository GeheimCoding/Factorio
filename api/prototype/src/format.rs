use crate::concept::Concept;
use crate::define::Define;
use crate::prototype::Prototype;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::Deserialize;
use std::path::Path;
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
        let path = Path::new("api/prototype/generated/defines");
        fs::create_dir_all(path)?;
        self.generate_defines(path)
    }

    fn generate_defines(&self, path: &Path) -> io::Result<()> {
        let results = self
            .defines
            .par_iter()
            .map(|define| define.generate(path))
            .collect::<Vec<_>>();
        results.into_iter().collect::<Result<_, _>>()
    }
}
