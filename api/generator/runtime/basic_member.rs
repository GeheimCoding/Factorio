#![allow(unused)]
use serde::Deserialize;

use crate::generator::{generate_docs, Generate, StringTransformation};

#[derive(Debug, Deserialize)]
pub struct BasicMember {
    /// The name of the member.
    name: String,
    /// The order of the member as shown in the HTML.
    order: u16,
    /// The text description of the member.
    description: String,
}

impl Generate for BasicMember {
    fn generate(
        &self,
        prefix: String,
        enum_variant: bool,
        indent: usize,
        unions: &mut Vec<String>,
    ) -> String {
        format!(
            "{}    {},\n",
            generate_docs(Some(&self.description), None, None, indent),
            self.name.to_pascal_case()
        )
    }
}
