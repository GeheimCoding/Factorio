#[derive(Debug, serde::Deserialize)]
pub struct AutoplaceControl {
    base_: crate::prototypes::Prototype,
    #[serde(default = "default_can_be_disabled")]
    can_be_disabled: bool,
    category: AutoplaceControlCategory,
    #[serde(default = "default_richness")]
    richness: bool,
}
fn default_can_be_disabled() -> bool {
    true
}
#[derive(Debug, serde::Deserialize)]
pub enum AutoplaceControlCategory {
    #[serde(rename = "resource")]
    Resource,
    #[serde(rename = "terrain")]
    Terrain,
    #[serde(rename = "cliff")]
    Cliff,
    #[serde(rename = "enemy")]
    Enemy,
}
fn default_richness() -> bool {
    false
}
