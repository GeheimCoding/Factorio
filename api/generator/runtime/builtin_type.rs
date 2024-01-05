#![allow(dead_code)]
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BuiltinType {
    /// The name of the built-in type.
    name: String,
    /// The order of the built-in type as shown in the HTML.
    order: u16,
    /// The text description of the built-in type.
    description: String,
}
