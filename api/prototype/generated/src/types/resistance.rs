#[derive(serde::Deserialize)]
pub struct Resistance {
    decrease: f32,
    percent: f32,
    #[serde(rename = "type")]
    type_: crate::types::DamageTypeID,
}
