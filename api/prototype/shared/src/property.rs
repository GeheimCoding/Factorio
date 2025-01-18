use crate::basic_member::BasicMember;
use crate::format::Context;
use crate::transformation::Transformation;
use crate::type_::{ComplexType, Type};
use serde::Deserialize;

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
    pub fn generate(&self, prefix: &str, context: &Context) -> (String, Vec<String>) {
        let prefix = format!("{prefix}{}", self.base.name.to_pascal_case());
        let (mut inner, additional) = self.type_.generate(&prefix, context);
        if self.type_.is_literal_value() {
            inner = String::from("String");
        }
        let (mut name, other) = self.base.name.to_rust_type(context);
        assert!(other.is_empty());
        name = name.to_snake_case();
        let rename = if self.base.name != name {
            &format!("#[serde(rename = \"{}\")]", self.base.name)
        } else {
            ""
        };
        let alias = if let Some(alt_name) = &self.alt_name {
            &format!("#[serde(alias = \"{alt_name}\")]")
        } else {
            ""
        };
        (format!("{rename}{alias}{name}: {inner}",), additional)
    }
}
