#[derive(Debug, serde::Deserialize)]
pub struct MaximumFollowingRobotsCountModifier {
    #[serde(flatten)]
    base_: crate::types::SimpleModifier,
    #[serde(default = "default_infer_icon")]
    infer_icon: bool,
    #[serde(default = "default_use_icon_overlay_constant")]
    use_icon_overlay_constant: bool,
}
fn default_infer_icon() -> bool {
    false
}
fn default_use_icon_overlay_constant() -> bool {
    true
}
