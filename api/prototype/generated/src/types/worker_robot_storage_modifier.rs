#[derive(Debug, serde::Deserialize)]
pub struct WorkerRobotStorageModifier {
    base_: crate::types::SimpleModifier,
    #[serde(default = "default_infer_icon")]
    infer_icon: bool,
    #[serde(default = "default_use_icon_overlay_constant")]
    use_icon_overlay_constant: bool,
}
fn default_infer_icon() -> bool {
    true
}
fn default_use_icon_overlay_constant() -> bool {
    true
}
