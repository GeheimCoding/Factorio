use crate::concept::Concept;
use crate::define::Define;
use crate::prototype::Prototype;
use serde::Deserialize;

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
