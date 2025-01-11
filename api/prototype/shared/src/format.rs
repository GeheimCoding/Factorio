use crate::concept::Concept;
use crate::custom_properties::CustomProperties;
use crate::define::Define;
use crate::property::Property;
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
pub struct Metadata<'a> {
    pub parent: &'a Option<String>,
    pub properties: Option<&'a Vec<Property>>,
    pub custom_properties: Option<&'a CustomProperties>,
}

#[derive(Debug)]
pub struct Context<'a> {
    pub context: HashMap<String, (Kind, DataType)>,
    pub inline_types: HashMap<String, &'a Concept>,
    pub metadata: HashMap<String, Metadata<'a>>,
}

impl Context<'_> {
    pub fn with_prefix(&self, rust_name: &str) -> (String, Vec<String>) {
        if let Some(define) = rust_name.split("defines.").into_iter().skip(1).next() {
            return (
                format!("crate::defines::{}", String::from(define).to_pascal_case()),
                vec![],
            );
        } else if rust_name
            .chars()
            .next()
            .expect("expected at least one char")
            .is_ascii_lowercase()
        {
            return (String::from(rust_name), vec![]);
        } else if let Some(inline_type) = self.inline_types.get(rust_name) {
            return (
                String::from(rust_name),
                vec![inline_type.generate_internal(self)],
            );
        }
        let (kind, _) = self
            .context
            .get(rust_name)
            .expect(&format!("expected context for {rust_name}"));

        (
            match kind {
                Kind::Concept => format!("crate::types::{rust_name}"),
                Kind::Prototype => format!("crate::prototypes::{rust_name}"),
            },
            vec![],
        )
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
        let mut inline_types = HashMap::new();
        let mut metadata = HashMap::new();

        for concept in self.types.iter() {
            let name = concept.rust_name();
            if let Some(found) = context.insert(
                String::from(name),
                (Kind::Concept, Self::get_datatype(&concept.type_)),
            ) {
                unreachable!("concept with name {name} already exists in context: {found:?}");
            }
            if concept.inline {
                if let Some(found) = inline_types.insert(String::from(name), concept) {
                    unreachable!(
                        "concept with name {name} already exists in inline types: {found:?}"
                    );
                }
            }
            if let Some(found) = metadata.insert(
                String::from(name),
                Metadata {
                    parent: &concept.parent,
                    properties: concept.properties.as_ref(),
                    custom_properties: None,
                },
            ) {
                unreachable!("concept with name {name} already exists in metadata: {found:?}");
            }
        }
        for prototype in &self.prototypes {
            let name = prototype.rust_name();
            if let Some(found) =
                context.insert(String::from(name), (Kind::Prototype, DataType::Struct))
            {
                unreachable!("prototype with name {name} already exists in context: {found:?}");
            }
            if let Some(found) = metadata.insert(
                String::from(name),
                Metadata {
                    parent: &prototype.parent,
                    properties: Some(prototype.properties.as_ref()),
                    custom_properties: prototype.custom_properties.as_ref(),
                },
            ) {
                unreachable!("concept with name {name} already exists in metadata: {found:?}");
            }
        }
        Context {
            context,
            inline_types,
            metadata,
        }
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
