#![allow(unused)]
use std::collections::HashSet;

use serde::Deserialize;

use crate::generator::{generate_docs, type_::Type, Generate, Macro, StringTransformation};

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

impl ParameterGroup {
    pub fn name(&self) -> String {
        if self.name == "Other types" {
            "OtherTypes".to_owned()
        } else {
            self.name.clone()
        }
    }
}

impl Generate for Parameter {
    fn generate(
        &self,
        prefix: String,
        enum_variant: bool,
        indent: usize,
        unions: &mut Vec<String>,
        class_names: &HashSet<String>,
    ) -> String {
        let name = self.name.clone().expect("should have a name");
        let new_prefix = format!("{prefix}{}", name.to_pascal_case());
        let mut result = generate_docs(Some(&self.description), None, None, None, indent);
        let type_ = self
            .type_
            .generate(new_prefix, enum_variant, indent, unions, class_names)
            .to_optional_if(self.optional);
        let name = name.to_rust_field_name();
        let name = if name == "_" {
            type_.chars().next().unwrap().to_lowercase().to_string()
        } else {
            name
        };
        let type_ = if prefix == type_ {
            format!("Box<{type_}>")
        } else {
            type_
        };
        result.push_str(&format!("    {}: {},", name, type_));
        result
    }
}

impl Generate for ParameterGroup {
    fn generate(
        &self,
        prefix: String,
        enum_variant: bool,
        indent: usize,
        unions: &mut Vec<String>,
        class_names: &HashSet<String>,
    ) -> String {
        let mut result = format!(
            "{}{}pub struct {prefix} {{\n{}\n",
            generate_docs(self.description.as_ref(), None, None, None, indent),
            Macro::DebugDeserialize.to_string(),
            self.parameters
                .iter()
                .map(|p| p.generate(
                    prefix.clone(),
                    enum_variant,
                    indent + 1,
                    unions,
                    class_names
                ))
                .collect::<Vec<_>>()
                .join("\n")
        );
        result.push('}');
        result
    }
}
