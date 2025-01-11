pub struct TechnologyPrototype {
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
pub enum TechnologyPrototypeMaxLevel {
    U32(u32),
    Infinite,
}
