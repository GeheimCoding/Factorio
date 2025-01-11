pub struct ArtilleryWagonPrototype {
    ammo_stack_limit: crate::types::ItemCountType,
    cannon_barrel_light_direction: crate::types::Vector3D,
    cannon_barrel_pictures: crate::types::RollingStockRotatedSlopedGraphics,
    cannon_barrel_recoil_shiftings: Vec<crate::types::Vector3D>,
    cannon_barrel_recoil_shiftings_load_correction_matrix: Vec<crate::types::Vector3D>,
    cannon_base_height: f64,
    cannon_base_pictures: crate::types::RollingStockRotatedSlopedGraphics,
    cannon_base_shift_when_horizontal: f64,
    cannon_base_shift_when_vertical: f64,
    cannon_parking_frame_count: u16,
    cannon_parking_speed: f32,
    disable_automatic_firing: bool,
    gun: crate::types::ItemID,
    inventory_size: crate::types::ItemStackIndex,
    manual_range_modifier: f64,
    rotating_sound: crate::types::InterruptibleSound,
    turn_after_shooting_cooldown: u16,
    turret_rotation_speed: f64,
}
