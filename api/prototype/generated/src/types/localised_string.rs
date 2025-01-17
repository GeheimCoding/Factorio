#[derive(serde::Deserialize)]
pub enum LocalisedString {
    #[serde(untagged)]
    String(String),
    #[serde(untagged)]
    VecLocalisedString(Vec<crate::types::LocalisedString>),
}
