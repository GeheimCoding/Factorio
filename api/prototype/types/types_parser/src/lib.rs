use anyhow::Context;
pub use generated::types::*;
use std::fs::File;
use zip::ZipArchive;

pub fn parse_type(input: &str) -> serde_json::Result<Types> {
    serde_json::from_str(input)
}

pub fn load_data_dump() -> anyhow::Result<Data> {
    let mut archive = ZipArchive::new(File::open("api/prototype/shared/data-raw-dump.zip")?)?;
    let json = archive.by_name("data-raw-dump.json")?;
    serde_json::from_reader(json).context("failed to load data")
}
