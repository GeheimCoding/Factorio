#[derive(Debug, serde::Deserialize)]
pub struct FluidTurretPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::TurretPrototype,
    activation_buffer_ratio: crate::types::FluidAmount,
    attacking_muzzle_animation_shift: Option<crate::types::AnimatedVector>,
    ending_attack_muzzle_animation_shift: Option<crate::types::AnimatedVector>,
    enough_fuel_indicator_light: Option<crate::types::LightDefinition>,
    enough_fuel_indicator_picture: Option<crate::types::Sprite4Way>,
    fluid_box: crate::types::FluidBox,
    fluid_buffer_input_flow: crate::types::FluidAmount,
    fluid_buffer_size: crate::types::FluidAmount,
    folded_muzzle_animation_shift: Option<crate::types::AnimatedVector>,
    folding_muzzle_animation_shift: Option<crate::types::AnimatedVector>,
    muzzle_animation: Option<crate::types::Animation>,
    muzzle_light: Option<crate::types::LightDefinition>,
    not_enough_fuel_indicator_light: Option<crate::types::LightDefinition>,
    not_enough_fuel_indicator_picture: Option<crate::types::Sprite4Way>,
    out_of_ammo_alert_icon: Option<crate::types::Sprite>,
    prepared_muzzle_animation_shift: Option<crate::types::AnimatedVector>,
    preparing_muzzle_animation_shift: Option<crate::types::AnimatedVector>,
    starting_attack_muzzle_animation_shift: Option<crate::types::AnimatedVector>,
    turret_base_has_direction: bool,
}
