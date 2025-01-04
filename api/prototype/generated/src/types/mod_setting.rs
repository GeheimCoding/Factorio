pub struct ModSetting {
    value: ModSettingValue,
}
pub enum ModSettingValue {
    I32(i32),
    F64(f64),
    Bool(bool),
    String(String),
    Color(crate::types::Color),
}
