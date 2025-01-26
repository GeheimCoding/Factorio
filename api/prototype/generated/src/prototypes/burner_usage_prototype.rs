#[derive(Debug, serde::Deserialize)]
pub struct BurnerUsagePrototype {
    base_: crate::prototypes::Prototype,
    accepted_fuel_key: String,
    burned_in_key: String,
    empty_slot_caption: crate::types::LocalisedString,
    empty_slot_description: Option<crate::types::LocalisedString>,
    empty_slot_sprite: crate::types::Sprite,
    icon: crate::types::Sprite,
    no_fuel_status: Option<crate::types::LocalisedString>,
}
