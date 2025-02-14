#[derive(Debug, serde::Deserialize)]
pub enum Mirroring {
    #[serde(rename = "horizontal")]
    Horizontal,
    #[serde(rename = "vertical")]
    Vertical,
    #[serde(rename = "diagonal-pos")]
    DiagonalPos,
    #[serde(rename = "diagonal-neg")]
    DiagonalNeg,
}
