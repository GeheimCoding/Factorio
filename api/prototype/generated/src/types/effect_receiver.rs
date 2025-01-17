#[derive(serde::Deserialize)]
pub struct EffectReceiver {
    base_effect: crate::types::Effect,
    uses_beacon_effects: bool,
    uses_module_effects: bool,
    uses_surface_effects: bool,
}
