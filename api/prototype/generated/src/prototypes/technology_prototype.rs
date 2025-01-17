pub struct TechnologyPrototype {
    base_: crate::prototypes::Prototype,
    allows_productivity: bool,
    effects: Vec<crate::types::Modifier>,
    enabled: bool,
    essential: bool,
    icon: crate::types::FileName,
    icon_size: crate::types::SpriteSizeType,
    icons: Vec<crate::types::IconData>,
    ignore_tech_cost_multiplier: bool,
    max_level: TechnologyPrototypeMaxLevel,
    name: String,
    prerequisites: Vec<crate::types::TechnologyID>,
    research_trigger: crate::types::TechnologyTrigger,
    unit: crate::types::TechnologyUnit,
    upgrade: bool,
    visible_when_disabled: bool,
}
#[derive(serde::Deserialize)]
pub enum TechnologyPrototypeMaxLevel {
    #[serde(rename = "infinite")]
    Infinite,
    #[serde(untagged)]
    U32(u32),
}
