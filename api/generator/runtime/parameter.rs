#![allow(unused)]
use serde::Deserialize;

use crate::generator::{generate_docs, type_::Type, Generate, StringTransformation};

#[derive(Debug, Deserialize)]
pub struct Parameter {
    /// The name of the parameter.
    name: Option<String>,
    /// The order of the parameter as shown in the HTML.
    order: u16,
    /// The text description of the parameter.
    description: String,
    /// The type of the parameter.
    #[serde(rename = "type")]
    type_: Type,
    /// Whether the type is optional or not.
    optional: bool,
}

#[derive(Debug, Deserialize)]
pub struct ParameterGroup {
    /// The name of the parameter group.
    name: String,
    /// The order of the parameter group as shown in the HTML.
    order: u16,
    /// The text description of the parameter group.
    description: Option<String>,
    /// The parameters that the group adds.
    parameters: Vec<Parameter>,
}

impl Generate for Parameter {
    fn generate(
        &self,
        prefix: String,
        enum_variant: bool,
        indent: usize,
        unions: &mut Vec<String>,
    ) -> String {
        let name = self.name.clone().expect("should have a name");
        let prefix = format!("{prefix}{}", name.to_pascal_case());
        let mut result = generate_docs(Some(&self.description), None, None, None, indent);
        result.push_str(&format!(
            "    {}: {},",
            name.to_rust_field_name(),
            self.type_
                .generate(prefix, enum_variant, indent, unions)
                .to_optional_if(self.optional)
        ));
        result
    }
}
