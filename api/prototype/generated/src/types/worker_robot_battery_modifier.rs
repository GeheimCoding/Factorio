#[derive(Debug, serde::Deserialize)]
pub struct WorkerRobotBatteryModifier {
    base_: crate::types::SimpleModifier,
    #[serde(default = "default_infer_icon")]
    infer_icon: bool,
    #[serde(rename = "type")]
    type_: String,
    #[serde(default = "default_use_icon_overlay_constant")]
    use_icon_overlay_constant: bool,
}
fn default_infer_icon() -> bool {
    true
}
fn default_use_icon_overlay_constant() -> bool {
    true
}
