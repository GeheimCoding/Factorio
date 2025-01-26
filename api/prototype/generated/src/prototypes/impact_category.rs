#[derive(Debug, serde::Deserialize)]
pub struct ImpactCategory {
    name: String,
    #[serde(rename = "type")]
    type_: String,
}
