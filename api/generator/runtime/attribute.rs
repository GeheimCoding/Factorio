#![allow(unused)]
use std::collections::{HashMap, HashSet};

use serde::Deserialize;

use crate::generator::{generate_docs, type_::Type, Generate, Macro, StringTransformation};

use super::event::EventRaised;

pub const LUA_ENTITY_SUBCLASSES: &[(&str, &[&str])] = &[
    ("build_distance", &["Character"]),
    ("character_additional_mining_categories", &["Character"]),
    ("character_build_distance_bonus", &["Character"]),
    ("character_crafting_speed_modifier", &["Character"]),
    ("character_health_bonus", &["Character"]),
    ("character_inventory_slots_bonus", &["Character"]),
    ("character_item_drop_distance_bonus", &["Character"]),
    ("character_item_pickup_distance_bonus", &["Character"]),
    ("character_loot_pickup_distance_bonus", &["Character"]),
    (
        "character_maximum_following_robot_count_bonus",
        &["Character"],
    ),
    ("character_mining_progress", &["Character"]),
    ("character_mining_speed_modifier", &["Character"]),
    (
        "character_personal_logistic_requests_enabled",
        &["Character"],
    ),
    ("character_reach_distance_bonus", &["Character"]),
    ("character_resource_reach_distance_bonus", &["Character"]),
    ("character_running_speed", &["Character"]),
    ("character_running_speed_modifier", &["Character"]),
    ("character_trash_slot_count_bonus", &["Character"]),
    ("cheat_mode", &["Character"]),
    ("cliff_orientation", &["Cliff"]),
    ("combat_robot_owner", &["CombatRobot"]),
    ("corpse_expires", &["Corpse"]),
    ("corpse_immune_to_entity_placement", &["Corpse"]),
    ("crafting_queue", &["Character"]),
    ("crafting_queue_progress", &["Character"]),
    ("crafting_queue_size", &["Character"]),
    ("cursor_ghost", &["Character"]),
    ("cursor_stack", &["Character"]),
    ("driving", &["Character"]),
    ("drop_item_distance", &["Character"]),
    ("following_robots", &["Character"]),
    ("in_combat", &["Character"]),
    ("inserter_stack_size_override", &["Inserter"]),
    ("inserter_target_pickup_count", &["Inserter"]),
    ("item_pickup_distance", &["Character"]),
    ("item_requests", &["Ghost", "ItemRequestProxy"]),
    ("link_id", &["LinkedContainer"]),
    ("loot_pickup_distance", &["Character"]),
    ("mining_state", &["Character"]),
    (
        "neighbours",
        &[
            "ElectricPole",
            "PowerSwitch",
            "UndergroundBelt",
            "Wall",
            "Gate",
            "Reactor",
            "Cliff",
            "PipeConnectable",
        ],
    ),
    ("opened", &["Character"]),
    ("opened_gui_type", &["Character"]),
    ("picking_state", &["Character"]),
    ("power_switch_state", &["PowerSwitch"]),
    ("proxy_target", &["ItemRequestProxy"]),
    ("reach_distance", &["Character"]),
    ("repair_state", &["Character"]),
    ("request_from_buffers", &["RequestSlot"]),
    ("resource_reach_distance", &["Character"]),
    ("riding_state", &["Character", "Player", "Car"]),
    ("rocket_silo_status", &["RocketSilo"]),
    ("selected", &["Character"]),
    ("shooting_state", &["Character"]),
    ("shooting_target", &["Turret"]),
    ("spawner", &["Unit"]),
    ("sticked_to", &["Sticker"]),
    ("storage_filter", &["LogisticStorageContainer"]),
    (
        "time_to_live",
        &["Ghost", "CombatRobot", "HighlightBox", "SmokeWithTrigger"],
    ),
    ("tree_color_index", &["Tree"]),
    ("tree_color_index_max", &["Tree"]),
    ("tree_gray_stage_index", &["Tree"]),
    ("tree_gray_stage_index_max", &["Tree"]),
    ("tree_stage_index", &["Tree"]),
    ("tree_stage_index_max", &["Tree"]),
    ("units", &["UnitSpawner"]),
    ("vehicle", &["Character"]),
    ("vehicle_logistic_requests_enabled", &["SpiderVehicle"]),
    ("walking_state", &["Character"]),
];

#[derive(Debug, Deserialize)]
pub struct Attribute {
    /// The name of the attribute.
    pub name: String,
    /// The order of the attribute as shown in the HTML.
    order: u16,
    /// The text description of the attribute.
    description: String,
    /// A list of strings containing additional information about the attribute.
    notes: Option<Vec<String>>,
    /// A list of strings containing example code and explanations.
    examples: Option<Vec<String>>,
    /// A list of events that this attribute might raise when written to.
    raises: Option<Vec<EventRaised>>,
    /// A list of strings specifying the sub-type (of the class) that the attribute applies to.
    pub subclasses: Option<Vec<String>>,
    /// The type of the attribute.
    #[serde(rename = "type")]
    pub type_: Type,
    /// Whether the attribute is optional or not.
    optional: bool,
    /// Whether the attribute can be read from.
    read: bool,
    /// Whether the attribute can be written to.
    write: bool,
}

impl Generate for Attribute {
    fn generate(
        &self,
        prefix: String,
        enum_variant: bool,
        indent: usize,
        unions: &mut Vec<String>,
        class_names: &HashSet<String>,
    ) -> String {
        let new_prefix = format!("{prefix}{}", self.name.to_pascal_case());
        let mut result = generate_docs(Some(&self.description), None, None, None, indent);
        let type_ = self
            .type_
            .generate(new_prefix, enum_variant, indent, unions, class_names);
        let nested_struct = format!("{}\npub struct", Macro::DebugDeserialize.to_string());
        let type_ = if type_.starts_with(&nested_struct) {
            let new_type = type_.split(&nested_struct).collect::<Vec<_>>()[1]
                .split_whitespace()
                .next()
                .unwrap()
                .to_owned();
            unions.push(type_);
            new_type
        } else {
            type_
        };
        let type_ = if type_ == prefix
            || type_.starts_with("LuaEntity")
            || type_ == "LuaInventory"
            || type_ == "LuaGui"
            || type_ == "LuaForce"
            || type_ == "LuaEquipmentPrototype"
            || type_ == "LuaBurnerOwner"
            || type_ == "Command"
            || type_ == "DragTarget"
        {
            format!("Box<{type_}>")
        } else if prefix == "MapSettings" && type_ == "SteeringMapSetting" {
            "SteeringMapSettings".to_owned()
        } else if (prefix == "LuaItemPrototype" && self.name == "flags")
            || (prefix == "LuaEntityPrototype" && self.name == "flags")
            || (prefix.ends_with("Settings") && !self.optional)
        {
            format!("Option<{type_}>")
        } else if prefix == "LuaParticlePrototype" && type_ == "TriggerEffectItem" {
            format!("Option<Vec<{type_}>>")
        } else {
            type_
        };
        let name = self.name.to_rust_field_name(enum_variant);
        if prefix == "LuaGameScript" && name == "pub backer_names" {
            result.push_str("    pub backer_names: HashMap<String, String>,");
        } else {
            result.push_str(&format!(
                "    {name}: {},",
                type_.to_optional_if(
                    self.optional
                        || self.subclasses.is_some()
                        || (prefix == "LuaItemStack"
                            && vec!["custom_description", "entity_filters", "tile_filters"]
                                .contains(&self.name.as_str()))
                        || ((prefix == "LuaEntity" || prefix == "LuaControl")
                            && (LUA_ENTITY_SUBCLASSES
                                .iter()
                                .any(|(attribute, _)| attribute == &self.name)
                                || vec!["drop_position", "logistic_cell", "logistic_network"]
                                    .contains(&self.name.as_str())))
                )
            ));
        }
        result
    }
}
