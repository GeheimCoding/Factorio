#[derive(serde::Deserialize)]
pub struct CharacterArmorAnimation {
    armors: Vec<crate::types::ItemID>,
    extra_smoke_cycles_per_tile: f32,
    flipped_shadow_running_with_gun: crate::types::RotatedAnimation,
    flying: crate::types::RotatedAnimation,
    flying_with_gun: crate::types::RotatedAnimation,
    idle: crate::types::RotatedAnimation,
    idle_in_air: crate::types::RotatedAnimation,
    idle_with_gun: crate::types::RotatedAnimation,
    idle_with_gun_in_air: crate::types::RotatedAnimation,
    landing: crate::types::RotatedAnimation,
    mining_with_tool: crate::types::RotatedAnimation,
    running: crate::types::RotatedAnimation,
    running_with_gun: crate::types::RotatedAnimation,
    smoke_cycles_per_tick: f32,
    smoke_in_air: Vec<crate::types::SmokeSource>,
    take_off: crate::types::RotatedAnimation,
}
