use crate::basic_member::BasicMember;
use crate::type_::Type;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RuntimeConcept {
    #[serde(flatten)]
    pub base: BasicMember,
    #[serde(rename = "type")]
    pub type_: Type,
}
