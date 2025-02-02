#[derive(Debug, serde::Deserialize)]
pub struct ArtilleryTurretPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityWithOwnerPrototype,
    #[serde(default = "default_alert_when_attacking")]
    alert_when_attacking: bool,
    ammo_stack_limit: crate::types::ItemCountType,
    automated_ammo_count: crate::types::ItemCountType,
    base_picture: Option<crate::types::Animation4Way>,
    #[serde(default = "default_base_picture_render_layer")]
    base_picture_render_layer: crate::types::RenderLayer,
    #[serde(default = "default_base_picture_secondary_draw_order")]
    base_picture_secondary_draw_order: u8,
    cannon_barrel_light_direction: Option<crate::types::Vector3D>,
    cannon_barrel_pictures: Option<crate::types::RotatedSprite>,
    cannon_barrel_recoil_shiftings: Option<crate::vec::Vec<crate::types::Vector3D>>,
    cannon_barrel_recoil_shiftings_load_correction_matrix:
        Option<crate::vec::Vec<crate::types::Vector3D>>,
    cannon_base_pictures: Option<crate::types::RotatedSprite>,
    cannon_base_shift: crate::types::Vector3D,
    #[serde(default = "default_cannon_parking_frame_count")]
    cannon_parking_frame_count: u16,
    #[serde(default = "default_cannon_parking_speed")]
    cannon_parking_speed: f32,
    circuit_connector: Option<crate::types::CircuitConnectorDefinition>,
    #[serde(default = "default_circuit_wire_max_distance")]
    circuit_wire_max_distance: f64,
    #[serde(default = "default_disable_automatic_firing")]
    disable_automatic_firing: bool,
    #[serde(default = "default_draw_circuit_wires")]
    draw_circuit_wires: bool,
    #[serde(default = "default_draw_copper_wires")]
    draw_copper_wires: bool,
    gun: crate::types::ItemID,
    inventory_size: crate::types::ItemStackIndex,
    #[serde(default = "default_is_military_target")]
    is_military_target: bool,
    manual_range_modifier: f64,
    rotating_sound: Option<crate::types::InterruptibleSound>,
    #[serde(default = "default_turn_after_shooting_cooldown")]
    turn_after_shooting_cooldown: u16,
    turret_rotation_speed: f64,
}
fn default_alert_when_attacking() -> bool {
    true
}
fn default_base_picture_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::LowerObject
}
fn default_base_picture_secondary_draw_order() -> u8 {
    0
}
fn default_cannon_parking_frame_count() -> u16 {
    0
}
fn default_cannon_parking_speed() -> f32 {
    1.0
}
fn default_circuit_wire_max_distance() -> f64 {
    0.0
}
fn default_disable_automatic_firing() -> bool {
    false
}
fn default_draw_circuit_wires() -> bool {
    true
}
fn default_draw_copper_wires() -> bool {
    true
}
fn default_is_military_target() -> bool {
    true
}
fn default_turn_after_shooting_cooldown() -> u16 {
    0
}
