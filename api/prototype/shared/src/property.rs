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
        let (mut inner, additional) = self.type_.generate(&prefix, &None, context);
        if self.type_.is_literal_value() {
            inner = String::from("String");
        }
        (
            format!("{}: {inner}", self.base.name.to_rust_type(context)),
            additional,
        )
    }
}
