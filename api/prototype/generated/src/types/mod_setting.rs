#[derive(Debug, serde::Deserialize)]
pub struct ModSetting {
    value: ModSettingValue,
}
#[derive(Debug, serde::Deserialize)]
pub enum ModSettingValue {
    #[serde(untagged)]
    I32(i32),
    #[serde(untagged)]
    F64(f64),
    #[serde(untagged)]
    Bool(bool),
    #[serde(untagged)]
    String(String),
    #[serde(untagged)]
    Color(crate::types::Color),
}
