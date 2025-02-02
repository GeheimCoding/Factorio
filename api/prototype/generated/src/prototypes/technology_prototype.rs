#[derive(Debug, serde::Deserialize)]
pub struct TechnologyPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::Prototype,
    #[serde(default = "default_allows_productivity")]
    allows_productivity: bool,
    effects: Option<Vec<crate::types::Modifier>>,
    #[serde(default = "default_enabled")]
    enabled: bool,
    #[serde(default = "default_essential")]
    essential: bool,
    icon: Option<crate::types::FileName>,
    #[serde(default = "default_icon_size")]
    icon_size: crate::types::SpriteSizeType,
    icons: Option<Vec<crate::types::IconData>>,
    #[serde(default = "default_ignore_tech_cost_multiplier")]
    ignore_tech_cost_multiplier: bool,
    max_level: Option<TechnologyPrototypeMaxLevel>,
    name: String,
    prerequisites: Option<Vec<crate::types::TechnologyID>>,
    research_trigger: Option<crate::types::TechnologyTrigger>,
    unit: Option<crate::types::TechnologyUnit>,
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
#[derive(Debug, serde::Deserialize)]
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
