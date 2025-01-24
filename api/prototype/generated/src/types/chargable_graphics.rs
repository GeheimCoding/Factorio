#[derive(serde::Deserialize)]
pub struct ChargableGraphics {
    charge_animation: Option<crate::types::Animation>,
    #[serde(default = "default_charge_animation_is_looped")]
    charge_animation_is_looped: bool,
    #[serde(default = "default_charge_cooldown")]
    charge_cooldown: u16,
    charge_light: Option<crate::types::LightDefinition>,
    discharge_animation: Option<crate::types::Animation>,
    #[serde(default = "default_discharge_cooldown")]
    discharge_cooldown: u16,
    discharge_light: Option<crate::types::LightDefinition>,
    picture: Option<crate::types::Sprite>,
}
fn default_charge_animation_is_looped() -> bool {
    true
}
fn default_charge_cooldown() -> u16 {
    0
}
fn default_discharge_cooldown() -> u16 {
    0
}
