#[derive(Debug, serde::Deserialize)]
pub enum ComparatorString {
    #[serde(rename = "=")]
    EqualTo,
    #[serde(rename = ">")]
    GreaterThan,
    #[serde(rename = "<")]
    LesserThan,
    #[serde(rename = ">=")]
    #[serde(alias = "≥")]
    GreaterThanOrEqualTo,
    #[serde(rename = "<=")]
    #[serde(alias = "≤")]
    LessThanOrEqualTo,
    #[serde(rename = "!=")]
    #[serde(alias = "≠")]
    NotEqualTo,
}
