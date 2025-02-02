#[derive(Debug, serde::Deserialize)]
pub enum LocalisedString {
    #[serde(untagged)]
    String(String),
    #[serde(untagged)]
    VecLocalisedString(crate::vec::Vec<crate::types::LocalisedString>),
}
