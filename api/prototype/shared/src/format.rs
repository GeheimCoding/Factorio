use crate::concept::Concept;
use crate::define::Define;
use crate::prototype::Prototype;
use serde::Deserialize;

// https://lua-api.factorio.com/stable/auxiliary/json-docs-prototype.html
#[derive(Debug, Deserialize)]
pub struct Format {
    pub application: String,
    pub application_version: String,
    pub api_version: u8,
    pub stage: String,
    pub prototypes: Vec<Prototype>,
    pub types: Vec<Concept>,
    pub defines: Vec<Define>,
}
