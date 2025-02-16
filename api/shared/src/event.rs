use crate::basic_member::BasicMember;
use crate::parameter::Parameter;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Event {
    #[serde(flatten)]
    pub base: BasicMember,
    pub data: Vec<Parameter>,
    pub filter: Option<String>,
}
