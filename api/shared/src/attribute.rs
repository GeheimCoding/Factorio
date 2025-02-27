use crate::basic_member::BasicMember;
use crate::event_raised::EventRaised;
use crate::type_::Type;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Attribute {
    #[serde(flatten)]
    pub base: BasicMember,
    pub visibility: Option<Vec<String>>,
    pub raises: Option<Vec<EventRaised>>,
    pub subclasses: Option<Vec<String>>,
    pub read_type: Option<Type>,
    pub write_type: Option<Type>,
    pub optional: bool,
}
