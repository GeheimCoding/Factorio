#[derive(Debug, serde::Deserialize)]
pub struct NightVisionEquipmentPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EquipmentPrototype,
    activate_sound: Option<crate::types::Sound>,
    color_lookup: crate::types::DaytimeColorLookupTable,
    #[serde(default = "default_darkness_to_turn_on")]
    darkness_to_turn_on: f32,
    deactivate_sound: Option<crate::types::Sound>,
    energy_input: crate::types::Energy,
}
fn default_darkness_to_turn_on() -> f32 {
    0.5
}
