#![allow(unused)]
use serde::Deserialize;

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
