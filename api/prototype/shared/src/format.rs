use crate::concept::Concept;
use crate::define::Define;
use crate::prototype::Prototype;
use crate::transformation::Transformation;
use crate::type_::Type;
use serde::Deserialize;
use std::collections::HashMap;

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

#[derive(Debug)]
pub struct Context(pub HashMap<String, (Kind, DataType)>);

impl Context {
    pub fn with_prefix(&self, rust_name: &str) -> String {
        if let Some(define) = rust_name.split("defines.").into_iter().skip(1).next() {
            return format!("crate::defines::{}", String::from(define).to_pascal_case());
        } else if rust_name
            .chars()
            .next()
            .expect("expected at least one char")
            .is_ascii_lowercase()
        {
            return String::from(rust_name);
        }
        let (kind, _) = self
            .0
            .get(rust_name)
            .expect(&format!("expected context for {rust_name}"));

        match kind {
            Kind::Concept => format!("crate::types::{rust_name}"),
            Kind::Prototype => format!("crate::prototypes::{rust_name}"),
        }
    }
}

#[derive(Debug)]
pub enum DataType {
    Union,
    Struct,
    NewType,
}

#[derive(Debug)]
pub enum Kind {
    Concept,
    Prototype,
}

impl Format {
    pub fn create_context(&self) -> Context {
        let mut context = HashMap::new();

        for concept in &self.types {
            let name = concept.rust_name();
            if let Some(found) = context.insert(
                String::from(name),
                (Kind::Concept, Self::get_datatype(&concept.type_)),
            ) {
                unreachable!("concept with name {name} already exists in context: {found:?}");
            }
        }
        for prototype in &self.prototypes {
            let name = prototype.rust_name();
            if let Some(found) =
                context.insert(String::from(name), (Kind::Prototype, DataType::Struct))
            {
                unreachable!("prototype with name {name} already exists in context: {found:?}");
            }
        }
        Context(context)
    }

    fn get_datatype(type_: &Type) -> DataType {
        if type_.is_struct() {
            DataType::Struct
        } else if type_.is_union() {
            DataType::Union
        } else {
            DataType::NewType
        }
    }
}
