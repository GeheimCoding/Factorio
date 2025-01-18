#[derive(serde::Deserialize)]
pub struct TechnologyPrototype {
    base_: crate::prototypes::Prototype,
    #[serde(default = "default_allows_productivity")]
    allows_productivity: bool,
    effects: Vec<crate::types::Modifier>,
    #[serde(default = "default_enabled")]
    enabled: bool,
    #[serde(default = "default_essential")]
    essential: bool,
    icon: crate::types::FileName,
    #[serde(default = "default_icon_size")]
    icon_size: crate::types::SpriteSizeType,
    icons: Vec<crate::types::IconData>,
    #[serde(default = "default_ignore_tech_cost_multiplier")]
    ignore_tech_cost_multiplier: bool,
    max_level: TechnologyPrototypeMaxLevel,
    name: String,
    prerequisites: Vec<crate::types::TechnologyID>,
    research_trigger: crate::types::TechnologyTrigger,
    unit: crate::types::TechnologyUnit,
    #[serde(default = "default_upgrade")]
    upgrade: bool,
    #[serde(default = "default_visible_when_disabled")]
    visible_when_disabled: bool,
}
fn default_allows_productivity() -> bool {
    true
}
fn default_enabled() -> bool {
    true
}
fn default_essential() -> bool {
    false
}
fn default_icon_size() -> crate::types::SpriteSizeType {
    64
}
fn default_ignore_tech_cost_multiplier() -> bool {
    false
}
#[derive(serde::Deserialize)]
pub enum TechnologyPrototypeMaxLevel {
    #[serde(rename = "infinite")]
    Infinite,
    #[serde(untagged)]
    U32(u32),
}
fn default_upgrade() -> bool {
    false
}
fn default_visible_when_disabled() -> bool {
    false
}
