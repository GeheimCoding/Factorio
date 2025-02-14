use crate::concept::Concept;
use crate::context::{Context, DataType, Kind, Metadata};
use crate::define::Define;
use crate::property::Property;
use crate::prototype::Prototype;
use crate::transformation::Transformation;
use crate::type_::{ComplexType, Type};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};

// https://lua-api.factorio.com/latest/auxiliary/json-docs-prototype.html
#[derive(Debug, Deserialize)]
pub struct PrototypeFormat {
    pub application: String,
    pub application_version: String,
    pub api_version: u8,
    pub stage: String,
    pub prototypes: Vec<Prototype>,
    pub types: Vec<Concept>,
    pub defines: Vec<Define>,
}

impl PrototypeFormat {
    pub fn create_context(&self) -> Context {
        let mut context = HashMap::new();
        let mut inline_types = HashMap::new();
        let mut metadata = HashMap::new();
        let mut hash_keys = HashSet::new();
        let mut overridden_properties = HashSet::new();

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
            let parent = concept.parent.as_ref();
            let properties = concept.properties.as_ref();
            self.find_overridden_properties(properties, &mut overridden_properties, parent);

            if let Some(found) = metadata.insert(
                String::from(name),
                Metadata {
                    parent,
                    properties,
                    custom_properties: None,
                    tagged_key: concept.get_tagged_key(),
                },
            ) {
                unreachable!("concept with name {name} already exists in metadata: {found:?}");
            }
            if let Some(properties) = properties {
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
            let parent = prototype.parent.as_ref();
            let properties = Some(&prototype.properties);
            self.find_overridden_properties(properties, &mut overridden_properties, parent);

            if let Some(found) = metadata.insert(
                String::from(name),
                Metadata {
                    parent,
                    properties,
                    custom_properties: prototype.custom_properties.as_ref(),
                    tagged_key: prototype.get_tagged_key(),
                },
            ) {
                unreachable!("concept with name {name} already exists in metadata: {found:?}");
            }
            hash_keys.extend(Self::extract_hash_keys(&prototype.properties, name));
        }
        Context {
            hash_keys,
            overridden_properties,
            metadata,
            context,
            inline_types,
        }
    }

    fn find_overridden_properties(
        &self,
        properties: Option<&Vec<Property>>,
        overridden_properties: &mut HashSet<String>,
        parent: Option<&String>,
    ) {
        if let Some(properties) = properties {
            for property in properties {
                if property.override_ {
                    self.add_overridden_properties(
                        overridden_properties,
                        &property.base.name,
                        parent.expect("expected parent for property with override flag"),
                    )
                }
            }
        }
    }

    fn add_overridden_properties(
        &self,
        overridden_properties: &mut HashSet<String>,
        property: &str,
        parent: &str,
    ) {
        overridden_properties.insert(format!("{parent}::{property}"));
        for concept in &self.types {
            if concept.base.name == parent {
                if let Some(parent) = concept.parent.as_ref() {
                    self.add_overridden_properties(overridden_properties, property, parent);
                }
                return;
            }
        }
        for prototype in &self.prototypes {
            if prototype.base.name == parent {
                if let Some(parent) = prototype.parent.as_ref() {
                    self.add_overridden_properties(overridden_properties, property, parent);
                }
                return;
            }
        }
        unreachable!("expected to find the parent type {parent}")
    }

    fn get_datatype(type_: &Type) -> DataType {
        if type_.is_struct() {
            DataType::Struct
        } else if type_.is_union() {
            DataType::Union
        } else {
            match type_ {
                Type::Simple(s) => DataType::NewType(String::from(s)),
                Type::Complex(_) => DataType::NewType(String::from("Complex")),
            }
        }
    }

    fn extract_hash_keys(properties: &[Property], prefix: &str) -> HashSet<String> {
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
