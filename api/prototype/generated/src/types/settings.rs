#[derive(serde::Deserialize)]
pub struct Settings {
    startup: std::collections::HashMap<String, crate::types::ModSetting>,
}
