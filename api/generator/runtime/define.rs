#![allow(unused)]
use serde::Deserialize;

use crate::generator::{generate_docs, Generate, StringTransformation};

use super::basic_member::BasicMember;

#[derive(Debug, Deserialize)]
pub struct Define {
    /// The name of the define.
    name: String,
    /// The order of the define as shown in the HTML.
    order: u16,
    /// The text description of the define.
    description: String,
    /// The members of the define.
    values: Option<Vec<BasicMember>>,
    /// A list of sub-defines.
    subkeys: Option<Vec<Define>>,
}

impl Generate for Define {
    fn generate(
        &self,
        prefix: String,
        enum_variant: bool,
        indent: usize,
        unions: &mut Vec<String>,
    ) -> String {
        let mut result = generate_docs(Some(&self.description), None, None, None, indent);
        result.push_str(&format!(
            "pub enum {}{} {{\n",
            prefix,
            self.name.to_pascal_case()
        ));
        if let Some(values) = self.values.as_ref() {
            result.push_str(
                &values
                    .iter()
                    .map(|v| v.generate(prefix.clone(), true, 1, unions))
                    .collect::<String>(),
            );
        }
        let mut unions = vec![];
        if let Some(subkeys) = self.subkeys.as_ref() {
            for define in subkeys {
                let prefix = format!("{prefix}{}", self.name.to_pascal_case());
                let name = define.name.to_pascal_case();
                let union = define.generate(prefix.clone(), enum_variant, indent, &mut unions);
                result.push_str(&format!("    {}({}{}),\n", name, prefix, name));
                unions.push(union);
            }
        }
        for union in unions {
            result.insert_str(0, &format!("{union}\n\n"));
        }
        result.push('}');
        result
    }
}
