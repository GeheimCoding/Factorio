#![allow(unused)]
use serde::Deserialize;

use crate::generator::{generate_docs, Generate, StringTransformation};

use super::parameter::Parameter;

#[derive(Debug, Deserialize)]
pub struct Event {
    /// The name of the event.
    name: String,
    /// The order of the event as shown in the HTML.
    order: u16,
    /// The text description of the event.
    description: String,
    /// A list of strings containing additional information about the event.
    notes: Option<Vec<String>>,
    /// A list of strings containing example code and explanations.
    examples: Option<Vec<String>>,
    /// The event-specific information that is provided.
    data: Vec<Parameter>,
}

#[derive(Debug, Deserialize)]
pub struct EventRaised {
    /// The name of the event being raised.
    name: String,
    /// The order of the member as shown in the HTML.
    order: u16,
    /// The text description of the raised event.
    description: String,
    /// The timeframe during which the event is raised. One of "instantly", "current_tick", or "future_tick".
    timeframe: String,
    /// Whether the event is always raised, or only dependant on a certain condition.
    optional: bool,
}

impl Generate for Event {
    fn generate(
        &self,
        prefix: String,
        enum_variant: bool,
        indent: usize,
        unions: &mut Vec<String>,
    ) -> String {
        let mut result = generate_docs(
            Some(&self.description),
            None,
            self.notes.as_ref(),
            self.examples.as_ref(),
            indent,
        );
        let name = self.name.to_pascal_case();
        result.push_str(&format!("pub struct {} {{\n", &name));
        result.push_str(
            &self
                .data
                .iter()
                .map(|p| p.generate(name.clone(), enum_variant, indent + 1, unions))
                .collect::<Vec<_>>()
                .join("\n"),
        );
        result.push_str("\n}");
        result
    }
}
