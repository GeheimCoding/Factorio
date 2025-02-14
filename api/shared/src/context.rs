use crate::concept::Concept;
use crate::custom_properties::CustomProperties;
use crate::property::Property;
use crate::transformation::Transformation;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Metadata<'a> {
    pub parent: Option<&'a String>,
    pub tagged_key: Option<&'a String>,
    pub properties: Option<&'a Vec<Property>>,
    pub custom_properties: Option<&'a CustomProperties>,
}

#[derive(Debug)]
pub struct Context<'a> {
    pub hash_keys: HashSet<String>,
    pub overridden_properties: HashSet<String>,
    pub metadata: HashMap<String, Metadata<'a>>,
    pub context: HashMap<String, (Kind, DataType)>,
    pub inline_types: HashMap<String, &'a Concept>,
}

impl Context<'_> {
    pub fn with_prefix(&self, rust_name: &str) -> (String, Vec<String>) {
        if let Some(define) = rust_name.split("defines.").nth(1) {
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
        // README: Adjustment [8]
        let rust_name = if rust_name == "DamageEntityTriggerEffectItem" {
            "DamageTriggerEffectItem"
        } else {
            rust_name
        };
        // README: Adjustment [8]
        let (kind, _) = self
            .context
            .get(rust_name)
            .unwrap_or_else(|| panic!("expected context for {rust_name}"));

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
    NewType(String),
}

#[derive(Debug)]
pub enum Kind {
    Concept,
    Prototype,
}
