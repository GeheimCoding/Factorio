#![allow(unused)]
use serde::Deserialize;

use super::{attribute::Attribute, method::Method, operator::Operator};

#[derive(Debug, Deserialize)]
pub struct Class {
    /// The name of the class.
    name: String,
    /// The order of the class as shown in the HTML.
    order: u16,
    /// The text description of the class.
    description: String,
    /// A list of strings containing additional information about the class.
    notes: Option<Vec<String>>,
    /// A list of strings containing example code and explanations.
    examples: Option<Vec<String>>,
    /// The methods that are part of the class.
    methods: Vec<Method>,
    /// The attributes that are part of the class.
    attributes: Vec<Attribute>,
    /// A list of operators on the class. They are called `call`, `index`, or `length` and have the format of either a [Method](https://lua-api.factorio.com/1.1.101/auxiliary/json-docs-runtime.html#Method) or an [Attribute](https://lua-api.factorio.com/1.1.101/auxiliary/json-docs-runtime.html#Attribute).
    operators: Vec<Operator>,
    /// Whether the class is never itself instantiated, only inherited from.
    #[serde(rename = "abstract")]
    abstract_: bool,
    /// A list of the names of the classes that his class inherits from.
    base_classes: Option<Vec<String>>,
}
