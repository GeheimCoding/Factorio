pub struct FluidTurretPrototype {
    activation_buffer_ratio: crate::types::FluidAmount,
    attack_parameters: crate::types::StreamAttackParameters,
    attacking_muzzle_animation_shift: crate::types::AnimatedVector,
    ending_attack_muzzle_animation_shift: crate::types::AnimatedVector,
    enough_fuel_indicator_light: crate::types::LightDefinition,
    enough_fuel_indicator_picture: crate::types::Sprite4Way,
    fluid_box: crate::types::FluidBox,
    fluid_buffer_input_flow: crate::types::FluidAmount,
    fluid_buffer_size: crate::types::FluidAmount,
    folded_muzzle_animation_shift: crate::types::AnimatedVector,
    folding_muzzle_animation_shift: crate::types::AnimatedVector,
    muzzle_animation: crate::types::Animation,
    muzzle_light: crate::types::LightDefinition,
    not_enough_fuel_indicator_light: crate::types::LightDefinition,
    not_enough_fuel_indicator_picture: crate::types::Sprite4Way,
    out_of_ammo_alert_icon: crate::types::Sprite,
    prepared_muzzle_animation_shift: crate::types::AnimatedVector,
    preparing_muzzle_animation_shift: crate::types::AnimatedVector,
    starting_attack_muzzle_animation_shift: crate::types::AnimatedVector,
    turret_base_has_direction: String,
}
