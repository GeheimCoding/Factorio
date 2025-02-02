#[derive(Debug, serde::Deserialize)]
pub struct CreateDecorativesTriggerEffectItem {
    #[serde(flatten)]
    base_: crate::types::TriggerEffectItem,
    #[serde(default = "default_apply_projection")]
    apply_projection: bool,
    decorative: crate::types::DecorativeID,
    #[serde(default = "default_radius_curve")]
    radius_curve: f32,
    spawn_max: u16,
    spawn_max_radius: f32,
    #[serde(default = "default_spawn_min")]
    spawn_min: u16,
    spawn_min_radius: f32,
    #[serde(default = "default_spread_evenly")]
    spread_evenly: bool,
}
fn default_apply_projection() -> bool {
    false
}
fn default_radius_curve() -> f32 {
    0.5
}
fn default_spawn_min() -> u16 {
    0
}
fn default_spread_evenly() -> bool {
    false
}
