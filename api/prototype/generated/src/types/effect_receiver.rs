#[derive(Debug, serde::Deserialize)]
pub struct EffectReceiver {
    base_effect: Option<crate::types::Effect>,
    #[serde(default = "default_uses_beacon_effects")]
    uses_beacon_effects: bool,
    #[serde(default = "default_uses_module_effects")]
    uses_module_effects: bool,
    #[serde(default = "default_uses_surface_effects")]
    uses_surface_effects: bool,
}
fn default_uses_beacon_effects() -> bool {
    true
}
fn default_uses_module_effects() -> bool {
    true
}
fn default_uses_surface_effects() -> bool {
    true
}
