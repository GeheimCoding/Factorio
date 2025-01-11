pub struct ArtilleryTurretPrototype {
    alert_when_attacking: bool,
    ammo_stack_limit: crate::types::ItemCountType,
    automated_ammo_count: crate::types::ItemCountType,
    base_picture: crate::types::Animation4Way,
    base_picture_render_layer: crate::types::RenderLayer,
    base_picture_secondary_draw_order: u8,
    cannon_barrel_light_direction: crate::types::Vector3D,
    cannon_barrel_pictures: crate::types::RotatedSprite,
    cannon_barrel_recoil_shiftings: Vec<crate::types::Vector3D>,
    cannon_barrel_recoil_shiftings_load_correction_matrix: Vec<crate::types::Vector3D>,
    cannon_base_pictures: crate::types::RotatedSprite,
    cannon_base_shift: crate::types::Vector3D,
    cannon_parking_frame_count: u16,
    cannon_parking_speed: f32,
    circuit_connector: crate::types::CircuitConnectorDefinition,
    circuit_wire_max_distance: f64,
    disable_automatic_firing: bool,
    draw_circuit_wires: bool,
    draw_copper_wires: bool,
    gun: crate::types::ItemID,
    inventory_size: crate::types::ItemStackIndex,
    is_military_target: bool,
    manual_range_modifier: f64,
    rotating_sound: crate::types::InterruptibleSound,
    turn_after_shooting_cooldown: u16,
    turret_rotation_speed: f64,
}