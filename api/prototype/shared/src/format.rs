use crate::concept::Concept;
use crate::custom_properties::CustomProperties;
use crate::define::Define;
use crate::property::Property;
use crate::prototype::Prototype;
use crate::transformation::Transformation;
use crate::type_::{ComplexType, Type};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};

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
    pub hash_keys: HashSet<String>,
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
        let mut hash_keys = HashSet::new();

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
            if let Some(properties) = &concept.properties {
                hash_keys.extend(Self::extract_hash_keys(properties, name));
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
            hash_keys.extend(Self::extract_hash_keys(&prototype.properties, name));
        }
        Context {
            context,
            inline_types,
            metadata,
            hash_keys,
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

    fn extract_hash_keys(properties: &Vec<Property>, prefix: &str) -> HashSet<String> {
        properties
            .iter()
            .flat_map(|p| {
                Self::get_hash_key(
                    &p.type_,
                    &format!("{prefix}_{}", p.base.name).to_pascal_case(),
                )
            })
            .collect()
    }

    fn get_hash_key(type_: &Type, name: &str) -> Option<String> {
        if let Type::Complex(complex_type) = type_ {
            if let ComplexType::Dictionary { key, .. } = complex_type.as_ref() {
                if let Type::Simple(simple) = key {
                    return Some(String::from(simple));
                } else {
                    return Some(String::from(name));
                }
            } else if let ComplexType::Type { value, .. } = complex_type.as_ref() {
                return Self::get_hash_key(value, name);
            }
        }
        None
    }
}
