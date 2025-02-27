use crate::class::Class;
use crate::context::Context;
use crate::define::Define;
use crate::event::Event;
use crate::method::Method;
use crate::parameter::Parameter;
use crate::runtime_concept::RuntimeConcept;
use serde::Deserialize;

// https://lua-api.factorio.com/latest/auxiliary/json-docs-runtime.html
#[derive(Debug, Deserialize)]
pub struct RuntimeFormat {
    pub application: String,
    pub application_version: String,
    pub api_version: u8,
    pub stage: String,
    pub classes: Vec<Class>,
    pub events: Vec<Event>,
    pub concepts: Vec<RuntimeConcept>,
    pub defines: Vec<Define>,
    pub global_objects: Vec<Parameter>,
    pub global_functions: Vec<Method>,
}

impl RuntimeFormat {
    pub fn create_context(&self) -> Context {
        Context {
            hash_keys: Default::default(),
            overridden_properties: Default::default(),
            metadata: Default::default(),
            context: Default::default(),
            inline_types: Default::default(),
        }
    }
}
