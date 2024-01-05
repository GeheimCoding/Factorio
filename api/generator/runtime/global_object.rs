#![allow(dead_code)]
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GlobalObject {
    /// The global variable name of the object.
    name: String,
    /// The order of the global object as shown in the HTML.
    order: u16,
    /// The text description of the global object.
    description: String,
    /// The class name of the global object.
    #[serde(rename = "type")]
    type_: String,
}
