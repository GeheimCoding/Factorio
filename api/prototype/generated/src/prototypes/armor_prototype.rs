pub struct ArmorPrototype {
    collision_box: crate::types::BoundingBox,
    drawing_box: crate::types::BoundingBox,
    equipment_grid: crate::types::EquipmentGridID,
    flight_sound: crate::types::InterruptibleSound,
    inventory_size_bonus: crate::types::ItemStackIndex,
    landing_sound: crate::types::Sound,
    moving_sound: crate::types::Sound,
    provides_flight: bool,
    resistances: Vec<crate::types::Resistance>,
    steps_sound: crate::types::Sound,
    takeoff_sound: crate::types::Sound,
}
