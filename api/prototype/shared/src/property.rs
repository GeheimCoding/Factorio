use crate::basic_member::BasicMember;
use crate::concept::Kind;
use crate::transformation::Transformation;
use crate::type_::{ComplexType, Type};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Property {
    #[serde(flatten)]
    pub base: BasicMember,
    pub visibility: Option<Vec<String>>,
    pub alt_name: Option<String>,
    #[serde(rename = "override")]
    pub override_: bool,
    #[serde(rename = "type")]
    pub type_: Type,
    pub optional: bool,
    pub default: Option<PropertyDefault>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum PropertyDefault {
    String(String),
    Literal(ComplexType),
}

impl Property {
    pub fn generate(&self, prefix: &str, kinds: &HashMap<String, Kind>) -> (String, Vec<String>) {
        let prefix = format!("{prefix}{}", self.base.name.to_pascal_case());
        let (inner, additional) = self.type_.generate(&prefix, &None, kinds);
        (
            format!("{}: {inner}", self.base.name.to_rust_type()),
            additional,
        )
    }
}
