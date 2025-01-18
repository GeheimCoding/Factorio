#[derive(serde::Deserialize)]
pub struct ArmorPrototype {
    base_: crate::prototypes::ToolPrototype,
    collision_box: crate::types::BoundingBox,
    drawing_box: crate::types::BoundingBox,
    equipment_grid: crate::types::EquipmentGridID,
    flight_sound: crate::types::InterruptibleSound,
    inventory_size_bonus: crate::types::ItemStackIndex,
    landing_sound: crate::types::Sound,
    moving_sound: crate::types::Sound,
    #[serde(default = "default_provides_flight")]
    provides_flight: bool,
    resistances: Vec<crate::types::Resistance>,
    steps_sound: crate::types::Sound,
    takeoff_sound: crate::types::Sound,
}
fn default_provides_flight() -> bool {
    false
}
