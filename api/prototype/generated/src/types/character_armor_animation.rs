#[derive(Debug, serde::Deserialize)]
pub struct CharacterArmorAnimation {
    armors: Option<crate::vec::Vec<crate::types::ItemID>>,
    #[serde(default = "default_extra_smoke_cycles_per_tile")]
    extra_smoke_cycles_per_tile: f32,
    flipped_shadow_running_with_gun: Option<crate::types::RotatedAnimation>,
    flying: Option<crate::types::RotatedAnimation>,
    flying_with_gun: Option<crate::types::RotatedAnimation>,
    idle: Option<crate::types::RotatedAnimation>,
    idle_in_air: Option<crate::types::RotatedAnimation>,
    idle_with_gun: crate::types::RotatedAnimation,
    idle_with_gun_in_air: Option<crate::types::RotatedAnimation>,
    landing: Option<crate::types::RotatedAnimation>,
    mining_with_tool: crate::types::RotatedAnimation,
    running: Option<crate::types::RotatedAnimation>,
    running_with_gun: crate::types::RotatedAnimation,
    #[serde(default = "default_smoke_cycles_per_tick")]
    smoke_cycles_per_tick: f32,
    smoke_in_air: Option<crate::vec::Vec<crate::types::SmokeSource>>,
    take_off: Option<crate::types::RotatedAnimation>,
}
fn default_extra_smoke_cycles_per_tile() -> f32 {
    0.0
}
fn default_smoke_cycles_per_tick() -> f32 {
    1.0
}
