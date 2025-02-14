#[derive(Debug, serde::Deserialize)]
pub enum SendToOrbitMode {
    #[serde(rename = "not-sendable")]
    NotSendable,
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "automated")]
    Automated,
}
