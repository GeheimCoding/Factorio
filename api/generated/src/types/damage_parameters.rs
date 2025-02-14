#[derive(Debug, serde::Deserialize)]
pub struct DamageParameters {
    amount: f32,
    #[serde(rename = "type")]
    type_: crate::types::DamageTypeID,
}
