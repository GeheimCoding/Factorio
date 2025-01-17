#[derive(serde::Deserialize)]
pub struct ChargableGraphics {
    charge_animation: crate::types::Animation,
    charge_animation_is_looped: bool,
    charge_cooldown: u16,
    charge_light: crate::types::LightDefinition,
    discharge_animation: crate::types::Animation,
    discharge_cooldown: u16,
    discharge_light: crate::types::LightDefinition,
    picture: crate::types::Sprite,
}
