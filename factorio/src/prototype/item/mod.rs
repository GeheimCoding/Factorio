use crate::prototype::InvalidPrototype;
use crate::{map_by_name, prototype};
use api::{ItemPrototype, Prototype};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum ItemVariant {
    // TODO: add data
    Ammo,
    Capsule,
    Gun,
    ItemWithEntityData,
    ItemWithLabel,
    ItemWithInventory,
    BlueprintBook,
    ItemWithTags,
    SelectionTool,
    BlueprintItem,
    CopyPasteTool,
    DeconstructionItem,
    UpgradeItem,
    Module,
    RailPlanner,
    SpidertronRemote,
    Tool,
    Armor,
    MiningTool,
    RepairTool,
}

#[derive(Clone, Debug)]
pub struct Item {
    pub name: String,
    pub stack_size: u32,
    pub variant: ItemVariant,
}

#[derive(Debug)]
pub struct Items {
    pub by_name: HashMap<String, Item>,
}

impl From<(&ItemPrototype, ItemVariant)> for Item {
    fn from((item, variant): (&ItemPrototype, ItemVariant)) -> Self {
        Self {
            name: item.parent_.name.clone(),
            stack_size: item.stack_size,
            variant,
        }
    }
}

impl TryFrom<&Prototype> for Item {
    type Error = InvalidPrototype;

    fn try_from(prototype: &Prototype) -> Result<Self, Self::Error> {
        use ItemVariant::*;
        Ok(if let Some(ammo) = prototype.as_ammo_item_prototype() {
            (&ammo.parent_, Ammo)
        } else if let Some(capsule) = prototype.as_capsule_prototype() {
            (&capsule.parent_, Capsule)
        } else if let Some(gun) = prototype.as_gun_prototype() {
            (&gun.parent_, Gun)
        } else if let Some(item_with_entity_data) = prototype.as_item_with_entity_data_prototype() {
            (&item_with_entity_data.parent_, ItemWithEntityData)
        } else if let Some(item_with_label) = prototype.as_item_with_label_prototype() {
            (&item_with_label.parent_, ItemWithLabel)
        } else if let Some(item_with_inventory) = prototype.as_item_with_inventory_prototype() {
            (&item_with_inventory.parent_.parent_, ItemWithInventory)
        } else if let Some(blueprint_book) = prototype.as_blueprint_book_prototype() {
            (&blueprint_book.parent_.parent_.parent_, BlueprintBook)
        } else if let Some(item_with_tags) = prototype.as_item_with_tags_prototype() {
            (&item_with_tags.parent_.parent_, ItemWithTags)
        } else if let Some(selection_tool) = prototype.as_selection_tool_prototype() {
            (&selection_tool.parent_.parent_, SelectionTool)
        } else if let Some(blueprint_item) = prototype.as_blueprint_item_prototype() {
            (&blueprint_item.parent_.parent_.parent_, BlueprintItem)
        } else if let Some(copy_paste_tool) = prototype.as_copy_paste_tool_prototype() {
            (&copy_paste_tool.parent_.parent_.parent_, CopyPasteTool)
        } else if let Some(deconstruction_item) = prototype.as_deconstruction_item_prototype() {
            (
                &deconstruction_item.parent_.parent_.parent_,
                DeconstructionItem,
            )
        } else if let Some(upgrade_item) = prototype.as_upgrade_item_prototype() {
            (&upgrade_item.parent_.parent_.parent_, UpgradeItem)
        } else if let Some(module) = prototype.as_module_prototype() {
            (&module.parent_, Module)
        } else if let Some(rail_planner) = prototype.as_rail_planner_prototype() {
            (&rail_planner.parent_, RailPlanner)
        } else if let Some(spidertron_remote) = prototype.as_spidertron_remote_prototype() {
            (&spidertron_remote.parent_, SpidertronRemote)
        } else if let Some(tool) = prototype.as_tool_prototype() {
            (&tool.parent_, Tool)
        } else if let Some(armor) = prototype.as_armor_prototype() {
            (&armor.parent_.parent_, Armor)
        } else if let Some(mining_tool) = prototype.as_mining_tool_prototype() {
            (&mining_tool.parent_.parent_, MiningTool)
        } else if let Some(repair_tool) = prototype.as_repair_tool_prototype() {
            (&repair_tool.parent_.parent_, RepairTool)
        } else {
            return Err(InvalidPrototype);
        }
        .into())
    }
}

impl From<&Vec<Prototype>> for Items {
    fn from(prototypes: &Vec<Prototype>) -> Self {
        let items = prototypes
            .iter()
            .filter_map(|prototype| Item::try_from(prototype).ok())
            .collect::<Vec<_>>();
        Self {
            by_name: map_by_name!(items),
        }
    }
}
