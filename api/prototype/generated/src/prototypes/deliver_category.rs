#[derive(serde::Deserialize)]
pub struct DeliverCategory {
    name: String,
    #[serde(rename = "type")]
    type_: String,
}
