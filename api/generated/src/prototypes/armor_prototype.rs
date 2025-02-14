#[derive(Debug, serde::Deserialize)]
pub struct ArmorPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::ToolPrototype,
    collision_box: Option<crate::types::BoundingBox>,
    drawing_box: Option<crate::types::BoundingBox>,
    equipment_grid: Option<crate::types::EquipmentGridID>,
    flight_sound: Option<crate::types::InterruptibleSound>,
    inventory_size_bonus: Option<crate::types::ItemStackIndex>,
    landing_sound: Option<crate::types::Sound>,
    moving_sound: Option<crate::types::Sound>,
    #[serde(default = "default_provides_flight")]
    provides_flight: bool,
    resistances: Option<crate::vec::Vec<crate::types::Resistance>>,
    steps_sound: Option<crate::types::Sound>,
    takeoff_sound: Option<crate::types::Sound>,
}
fn default_provides_flight() -> bool {
    false
}
