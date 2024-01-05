#![allow(unused)]
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BasicMember {
    /// The name of the member.
    name: String,
    /// The order of the member as shown in the HTML.
    order: u16,
    /// The text description of the member.
    description: String,
}
