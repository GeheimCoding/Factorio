#![allow(unused)]
use std::collections::HashSet;

use serde::Deserialize;

use crate::generator::{
    generate_docs,
    type_::{self, Type},
    Generate, Macro, StringTransformation,
};

use super::image::Image;

#[derive(Debug, Deserialize)]
pub struct Property {
    /// The name of the property.
    name: String,
    /// The order of the property as shown in the HTML.
    order: u16,
    /// The text description of the property.
    description: String,
    /// A list of Markdown lists to provide additional information. Usually contained in a spoiler tag.
    lists: Option<Vec<String>>,
    /// A list of code-only examples about the property.
    examples: Option<Vec<String>>,
    /// A list of illustrative images shown next to the property.
    images: Option<Vec<Image>>,
    /// An alternative name for the property. Either this or name can be used to refer to the property.
    alt_name: Option<String>,
    /// Whether the property overrides a property of the same name in one of its parents.
    #[serde(rename = "override")]
    override_: bool,
    /// The type of the property.
    #[serde(rename = "type")]
    type_: Type,
    /// Whether the property is optional and can be omitted. If so, it falls back to a default value.
    optional: bool,
    /// default :: union[string, Literal] (optional): The default value of the property. Either a textual description or a literal value.
    default: Option<PropertyDefaultUnion>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum PropertyDefaultUnion {
    String(String),
    Literal(Type),
}

#[derive(Debug, Deserialize)]
pub struct CustomProperties {
    /// The text description of the property.
    description: String,
    /// A list of Markdown lists to provide additional information. Usually contained in a spoiler tag.
    lists: Option<Vec<String>>,
    /// A list of code-only examples about the property.
    examples: Option<Vec<String>>,
    /// A list of illustrative images shown next to the property.
    images: Option<Vec<Image>>,
    /// The type of the key of the custom property.
    key_type: Type,
    /// The type of the value of the custom property.
    value_type: Type,
}

impl Generate for Property {
    fn generate(
        &self,
        prefix: String,
        enum_variant: bool,
        indent: usize,
        unions: &mut Vec<String>,
        class_names: &HashSet<String>,
    ) -> String {
        // TODO: default?
        let type_ = self.type_.generate(
            format!("{prefix}{}", self.name.to_pascal_case()),
            enum_variant,
            indent,
            unions,
            class_names,
        );
        let type_ = if type_ == prefix {
            format!("Box<{type_}>")
        } else {
            type_
        };
        let serde = if let Some(alt_name) = &self.alt_name {
            Macro::Alias(alt_name.clone()).to_string()
        } else {
            String::new()
        };
        let docs = generate_docs(
            Some(&self.description),
            self.lists.as_ref(),
            None,
            self.examples.as_ref(),
            indent,
        );
        if prefix == "CraftingMachinePrototype" && self.name == "fluid_boxes" {
            unions.push(format!(
                "
                #[derive(Debug, Deserialize, EnumAsInner, Traversable)]
                #[serde(untagged)]
                pub enum CraftingMachinePrototypeFluidBoxes {{
                    Vec(Vec<CraftingMachinePrototypeFluidBoxesValue>),
                    Map(HashMap<String, CraftingMachinePrototypeFluidBoxesValue>),
                }}
            "
            ));
            unions.push(format!(
                "
                #[derive(Debug, Deserialize, EnumAsInner, Traversable)]
                #[serde(untagged)]
                pub enum CraftingMachinePrototypeFluidBoxesValue {{
                    FluidBox(FluidBox),
                    Bool(bool),
                }}
            "
            ));
            format!("{docs}pub fluid_boxes: Option<CraftingMachinePrototypeFluidBoxes>,")
        } else if prefix == "HeatConnection" && self.name == "direction" {
            unions.push(format!(
                "
                #[derive(Debug, Deserialize, EnumAsInner, Traversable)]
                #[serde(untagged)]
                pub enum HeatConnectionDirection {{
                    Direction(Direction),
                    Number(u8),
                }}
            "
            ));
            format!("{docs}pub direction: HeatConnectionDirection,")
        } else if prefix == "BaseAttackParameters" && self.name == "sound" {
            unions.push(format!(
                "
                #[derive(Debug, Deserialize, EnumAsInner, Traversable)]
                #[serde(untagged)]
                pub enum BaseAttackParametersSound {{
                    LayeredSound(LayeredSound),
                    Map(HashMap<String, Sound>),
                }}
            "
            ));
            format!("{docs}pub sound: Option<BaseAttackParametersSound>,")
        } else if prefix == "ItemWithInventoryPrototype" && self.name == "inventory_size" {
            unions.push(format!(
                "
                #[derive(Debug, Deserialize, EnumAsInner, Traversable)]
                #[serde(untagged)]
                pub enum ItemWithInventoryPrototypeInventorySize {{
                    ItemStackIndex(ItemStackIndex),
                    String(String),
                }}
            "
            ));
            format!("{docs}pub inventory_size: ItemWithInventoryPrototypeInventorySize,")
        } else if prefix == "EntityPrototype" && self.name == "selection_box" {
            unions.push(format!(
                "
                #[derive(Debug, Deserialize, EnumAsInner, Traversable)]
                #[serde(untagged)]
                pub enum EntityPrototypeBoundingBox {{
                    BoundingBox(BoundingBox),
                    BoundingBoxExtra(BoundingBoxExtra),
                }}
            "
            ));
            format!("{docs}pub selection_box: Option<EntityPrototypeBoundingBox>,")
        } else if prefix == "EntityPrototype" && self.name == "collision_box" {
            format!("{docs}pub collision_box: Option<EntityPrototypeBoundingBox>,")
        } else if prefix == "OrientedCliffPrototype" && self.name == "collision_bounding_box" {
            format!("{docs}pub collision_bounding_box: EntityPrototypeBoundingBox,")
        } else if prefix == "MinableProperties" && self.name == "results" {
            format!("{docs}pub results: Option<TypeOrEmptyMap<Vec<ProductPrototype>>>,")
        } else if prefix == "EntityWithHealthPrototype" && self.name == "resistances" {
            format!("{docs}pub resistances: Option<TypeOrEmptyMap<Vec<Resistance>>>,")
        } else if prefix == "EntityPrototype" && self.name == "collision_mask" {
            format!("{docs}pub collision_mask: Option<TypeOrEmptyMap<CollisionMask>>,")
        } else if prefix == "EntityPrototype" && self.name == "flags" {
            format!("{docs}pub flags: Option<TypeOrEmptyMap<EntityPrototypeFlags>>,")
        } else if prefix == "TilePrototype" && self.name == "collision_mask" {
            format!("{docs}pub collision_mask: TypeOrEmptyMap<CollisionMask>,")
        } else if prefix == "MiningDrillPrototype" && self.name == "allowed_effects" {
            format!("{docs}pub allowed_effects: Option<TypeOrEmptyMap<EffectTypeLimitation>>,")
        } else if prefix == "FootprintParticle" && self.name == "tiles" {
            format!("{docs}pub tiles: TypeOrEmptyMap<Vec<TileID>>,")
        } else if (prefix == "CraftingMachinePrototype" || prefix == "MiningDrillGraphicsSet")
            && self.name == "shift_animation_waypoint_stop_duration"
        {
            format!("{docs}pub shift_animation_waypoint_stop_duration: Option<Float>,")
        } else if prefix == "SpriteParameters" && self.name == "mipmap_count" {
            format!("{docs}pub mipmap_count: Option<Float>,")
        } else if prefix == "SpiderVehicleGraphicsSet" && self.name == "light_positions" {
            format!("{docs}pub light_positions: Option<HashMap<String, Vec<Vector>>>,")
        } else if self.override_ {
            format!(
                "{}/* <already in base class> {}: {}, */",
                docs.replace("///", "//"),
                self.name.to_rust_field_name(enum_variant),
                type_.to_optional_if(self.optional)
            )
        } else {
            format!(
                "{docs}{}{}{}: {},",
                "    ".repeat(indent),
                serde,
                self.name.to_rust_field_name(enum_variant),
                type_.to_optional_if(
                    self.optional
                        || (prefix == "SpriteParameters" && self.name == "filename")
                        || ((prefix == "RotatedAnimation" || prefix == "RotatedSprite")
                            && self.name == "direction_count")
                        || (prefix == "SelectionToolPrototype"
                            && (self.name == "alt_selection_mode"
                                || self.name == "selection_mode"))
                        || (prefix == "CreateParticleTriggerEffectItem"
                            && (self.name == "initial_height"
                                || self.name == "particle_name"
                                || self.name == "type"))
                        || (prefix == "TileTransitions"
                            && (self.name == "inner_corner"
                                || self.name == "outer_corner"
                                || self.name == "inner_corner_mask"
                                || self.name == "outer_corner_mask"
                                || self.name == "side"
                                || self.name == "side_mask"))
                        || (prefix == "ContainerPrototype" && self.name == "picture")
                        || (prefix == "CreateDecorativesTriggerEffectItem" && self.name == "type")
                        || (prefix == "RecipeData" && self.name == "results")
                        || (prefix == "ItemProductPrototype"
                            && (self.name == "amount_max" || self.name == "amount_min"))
                        || (prefix == "TurretPrototype" && self.name == "attack_parameters")
                        || (prefix == "LogisticContainerPrototype" && self.name == "logistic_mode")
                        || (prefix == "IconData" && self.name == "icon_size")
                        || (prefix == "DontUseEntityInEnergyProductionAchievementPrototype"
                            && self.name == "included")
                )
            )
        }
    }
}
