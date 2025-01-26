#[derive(Debug, serde::Deserialize)]
pub struct ArtilleryWagonPrototype {
    base_: crate::prototypes::RollingStockPrototype,
    ammo_stack_limit: crate::types::ItemCountType,
    cannon_barrel_light_direction: Option<crate::types::Vector3D>,
    cannon_barrel_pictures: Option<crate::types::RollingStockRotatedSlopedGraphics>,
    cannon_barrel_recoil_shiftings: Option<Vec<crate::types::Vector3D>>,
    cannon_barrel_recoil_shiftings_load_correction_matrix: Option<Vec<crate::types::Vector3D>>,
    #[serde(default = "default_cannon_base_height")]
    cannon_base_height: f64,
    cannon_base_pictures: Option<crate::types::RollingStockRotatedSlopedGraphics>,
    #[serde(default = "default_cannon_base_shift_when_horizontal")]
    cannon_base_shift_when_horizontal: f64,
    #[serde(default = "default_cannon_base_shift_when_vertical")]
    cannon_base_shift_when_vertical: f64,
    #[serde(default = "default_cannon_parking_frame_count")]
    cannon_parking_frame_count: u16,
    #[serde(default = "default_cannon_parking_speed")]
    cannon_parking_speed: f32,
    #[serde(default = "default_disable_automatic_firing")]
    disable_automatic_firing: bool,
    gun: crate::types::ItemID,
    inventory_size: crate::types::ItemStackIndex,
    manual_range_modifier: f64,
    rotating_sound: Option<crate::types::InterruptibleSound>,
    #[serde(default = "default_turn_after_shooting_cooldown")]
    turn_after_shooting_cooldown: u16,
    turret_rotation_speed: f64,
}
fn default_cannon_base_height() -> f64 {
    0.0
}
fn default_cannon_base_shift_when_horizontal() -> f64 {
    0.0
}
fn default_cannon_base_shift_when_vertical() -> f64 {
    0.0
}
fn default_cannon_parking_frame_count() -> u16 {
    0
}
fn default_cannon_parking_speed() -> f32 {
    1.0
}
fn default_disable_automatic_firing() -> bool {
    false
}
fn default_turn_after_shooting_cooldown() -> u16 {
    0
}
