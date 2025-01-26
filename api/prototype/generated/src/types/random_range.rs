#[derive(Debug, serde::Deserialize)]
pub enum RandomRange {
    #[serde(untagged)]
    U8U8((u8, u8)),
    #[serde(untagged)]
    U8(u8),
}
