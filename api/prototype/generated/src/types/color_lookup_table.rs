#[derive(Debug, serde::Deserialize)]
pub enum ColorLookupTable {
    #[serde(rename = "identity")]
    Identity,
    #[serde(untagged)]
    FileName(crate::types::FileName),
}
