use crate::basic_member::BasicMember;
use crate::define_value::DefineValue;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Define {
    #[serde(flatten)]
    base: BasicMember,
    values: Option<Vec<DefineValue>>,
    subkeys: Option<Vec<Define>>,
}
