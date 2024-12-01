pub struct ChargableGraphics {
    charge_animation: Animation,
    charge_animation_is_looped: bool,
    charge_cooldown: u16,
    charge_light: LightDefinition,
    discharge_animation: Animation,
    discharge_cooldown: u16,
    discharge_light: LightDefinition,
    picture: Sprite,
}
