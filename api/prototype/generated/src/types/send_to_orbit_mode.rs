#[derive(serde::Deserialize)]
pub enum SendToOrbitMode {
    #[serde(rename = "not_sendable")]
    NotSendable,
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "automated")]
    Automated,
}
